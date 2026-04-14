use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct WasmTreasuryStatus {
    pub treasury_balance_raw: String,
    pub stable_decimals: u8,
    pub stable_symbol: String,
    pub total_supply_raw: String,
    pub circulating_supply_raw: String,
    pub treasury_held_tokens_raw: String,
    pub token_decimals: u8,
    pub token_symbol: String,
    pub floor_price_raw_1e18: String,
    pub current_month: u64,
}

#[cfg(feature = "web")]
mod web_rpc {
    use super::WasmTreasuryStatus;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, Response};

    const SEL_STABLE_TOKEN: [u8; 4] = [0xa9, 0xd7, 0x5b, 0x2b]; // stableToken()
    const SEL_BALANCE_OF: [u8; 4] = [0x70, 0xa0, 0x82, 0x31]; // balanceOf(address)
    const SEL_DECIMALS: [u8; 4] = [0x31, 0x3c, 0xe5, 0x67]; // decimals()
    const SEL_SYMBOL: [u8; 4] = [0x95, 0xd8, 0x9b, 0x41]; // symbol()
    const SEL_TOTAL_SUPPLY: [u8; 4] = [0x18, 0x16, 0x0d, 0xdd]; // totalSupply()
    const SEL_CIRCULATING: [u8; 4] = [0x93, 0x58, 0x92, 0x8b]; // circulatingSupply()
    const SEL_GET_PRICE: [u8; 4] = [0x98, 0xd5, 0xfd, 0xca]; // getPrice()
    const SEL_CURRENT_MONTH: [u8; 4] = [0x86, 0x2a, 0x4d, 0x47]; // currentMonth()

    fn bytes_to_hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            s.push(HEX_CHARS[(b >> 4) as usize]);
            s.push(HEX_CHARS[(b & 0x0f) as usize]);
        }
        s
    }

    const HEX_CHARS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    fn hex_char_to_u8(c: u8) -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => 0,
        }
    }

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        let bytes = hex.as_bytes();
        let mut result = Vec::with_capacity(bytes.len() / 2);
        let mut i = 0;
        while i + 1 < bytes.len() {
            result.push((hex_char_to_u8(bytes[i]) << 4) | hex_char_to_u8(bytes[i + 1]));
            i += 2;
        }
        result
    }

    fn encode_call(selector: &[u8; 4], args: &[&str]) -> String {
        let mut data = Vec::with_capacity(4 + args.len() * 32);
        data.extend_from_slice(selector);
        for arg in args {
            let arg_bytes = hex_to_bytes(arg);
            let mut padded = vec![0u8; 32];
            if arg_bytes.len() <= 32 {
                padded[32 - arg_bytes.len()..].copy_from_slice(&arg_bytes);
            }
            data.extend_from_slice(&padded);
        }
        format!("0x{}", bytes_to_hex(&data))
    }

    fn decode_uint256(hex: &str) -> u128 {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        if hex.len() < 64 {
            return 0;
        }
        let word = &hex[..64];
        u128::from_str_radix(&word[word.len().saturating_sub(32)..], 16).unwrap_or(0)
    }

    fn decode_uint8(hex: &str) -> u8 {
        decode_uint256(hex) as u8
    }

    fn decode_uint64(hex: &str) -> u64 {
        decode_uint256(hex) as u64
    }

    fn decode_string(hex: &str) -> String {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        if hex.len() < 128 {
            return String::new();
        }
        let len =
            u64::from_str_radix(&hex[64..128].trim_start_matches('0'), 16).unwrap_or(0) as usize;
        if len == 0 {
            return String::new();
        }
        let data_start = 128;
        let data_end = data_start + len * 2;
        if hex.len() < data_end {
            return String::new();
        }
        let bytes = hex_to_bytes(&hex[data_start..data_end]);
        String::from_utf8(bytes).unwrap_or_default()
    }

    fn decode_address(hex: &str) -> String {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        if hex.len() < 64 {
            return String::new();
        }
        format!("0x{}", &hex[24..64])
    }

    fn get_rpc_url(chain_id: u64) -> Result<&'static str, String> {
        match chain_id {
            1001 => Ok("https://public-en-kairos.node.kaia.io"),
            8217 => Ok("https://public-en.node.kaia.io"),
            _ => Err(format!("unsupported chain_id: {chain_id}")),
        }
    }

    async fn eth_call(rpc_url: &str, to: &str, data: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{"to": to, "data": data}, "latest"],
            "id": 1
        });

        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(
            &wasm_bindgen::JsValue::from_str(&body.to_string()),
        ));

        let request =
            Request::new_with_str_and_init(rpc_url, &opts).map_err(|e| format!("{e:?}"))?;
        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|e| format!("{e:?}"))?;

        let window = web_sys::window().ok_or("no window")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| format!("fetch failed: {e:?}"))?;

        let resp: Response = resp_value.dyn_into().map_err(|e| format!("{e:?}"))?;
        let json = JsFuture::from(resp.json().map_err(|e| format!("{e:?}"))?)
            .await
            .map_err(|e| format!("json parse failed: {e:?}"))?;

        let parsed: serde_json::Value =
            serde_wasm_bindgen::from_value(json).map_err(|e| format!("deser failed: {e:?}"))?;

        if let Some(err) = parsed.get("error") {
            return Err(format!("RPC error: {err}"));
        }

        parsed
            .get("result")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "missing result field".to_string())
    }

    pub async fn fetch_treasury_status(
        chain_id: u64,
        treasury_address: &str,
        brand_token_address: &str,
    ) -> Result<WasmTreasuryStatus, String> {
        let rpc_url = get_rpc_url(chain_id)?;

        let stable_call = encode_call(&SEL_STABLE_TOKEN, &[]);
        let stable_result = eth_call(rpc_url, treasury_address, &stable_call).await?;
        let stable_address = decode_address(&stable_result);

        let treasury_addr_hex = treasury_address
            .strip_prefix("0x")
            .unwrap_or(treasury_address);
        let stable_balance_call = encode_call(&SEL_BALANCE_OF, &[treasury_addr_hex]);
        let stable_decimals_call = encode_call(&SEL_DECIMALS, &[]);
        let stable_symbol_call = encode_call(&SEL_SYMBOL, &[]);

        let (stable_balance_result, stable_decimals_result, stable_symbol_result) = futures_join3(
            eth_call(rpc_url, &stable_address, &stable_balance_call),
            eth_call(rpc_url, &stable_address, &stable_decimals_call),
            eth_call(rpc_url, &stable_address, &stable_symbol_call),
        )
        .await;

        let treasury_balance_raw = decode_uint256(&stable_balance_result?);
        let stable_decimals = decode_uint8(&stable_decimals_result?);
        let stable_symbol = decode_string(&stable_symbol_result?);

        let total_supply_call = encode_call(&SEL_TOTAL_SUPPLY, &[]);
        let circulating_call = encode_call(&SEL_CIRCULATING, &[]);
        let token_balance_call = encode_call(&SEL_BALANCE_OF, &[treasury_addr_hex]);
        let get_price_call = encode_call(&SEL_GET_PRICE, &[]);
        let token_decimals_call = encode_call(&SEL_DECIMALS, &[]);
        let token_symbol_call = encode_call(&SEL_SYMBOL, &[]);
        let current_month_call = encode_call(&SEL_CURRENT_MONTH, &[]);

        let (
            total_supply_result,
            circulating_result,
            token_balance_result,
            get_price_result,
            token_decimals_result,
            token_symbol_result,
            current_month_result,
        ) = futures_join7(
            eth_call(rpc_url, brand_token_address, &total_supply_call),
            eth_call(rpc_url, brand_token_address, &circulating_call),
            eth_call(rpc_url, brand_token_address, &token_balance_call),
            eth_call(rpc_url, treasury_address, &get_price_call),
            eth_call(rpc_url, brand_token_address, &token_decimals_call),
            eth_call(rpc_url, brand_token_address, &token_symbol_call),
            eth_call(rpc_url, treasury_address, &current_month_call),
        )
        .await;

        let total_supply_raw = decode_uint256(&total_supply_result?);
        let circulating_supply_raw = decode_uint256(&circulating_result?);
        let treasury_held_tokens_raw = decode_uint256(&token_balance_result?);
        let floor_price_raw_1e18 = decode_uint256(&get_price_result?);
        let token_decimals = decode_uint8(&token_decimals_result?);
        let token_symbol = decode_string(&token_symbol_result?);
        let current_month = decode_uint64(&current_month_result?);

        Ok(WasmTreasuryStatus {
            treasury_balance_raw: treasury_balance_raw.to_string(),
            stable_decimals,
            stable_symbol,
            total_supply_raw: total_supply_raw.to_string(),
            circulating_supply_raw: circulating_supply_raw.to_string(),
            treasury_held_tokens_raw: treasury_held_tokens_raw.to_string(),
            token_decimals,
            token_symbol,
            floor_price_raw_1e18: floor_price_raw_1e18.to_string(),
            current_month,
        })
    }

    async fn futures_join3<A, B, C>(
        a: impl std::future::Future<Output = A>,
        b: impl std::future::Future<Output = B>,
        c: impl std::future::Future<Output = C>,
    ) -> (A, B, C) {
        let a = a.await;
        let b = b.await;
        let c = c.await;
        (a, b, c)
    }

    async fn futures_join7<A, B, C, D, E, F, G>(
        a: impl std::future::Future<Output = A>,
        b: impl std::future::Future<Output = B>,
        c: impl std::future::Future<Output = C>,
        d: impl std::future::Future<Output = D>,
        e: impl std::future::Future<Output = E>,
        f: impl std::future::Future<Output = F>,
        g: impl std::future::Future<Output = G>,
    ) -> (A, B, C, D, E, F, G) {
        let a = a.await;
        let b = b.await;
        let c = c.await;
        let d = d.await;
        let e = e.await;
        let f = f.await;
        let g = g.await;
        (a, b, c, d, e, f, g)
    }
}

#[cfg(feature = "web")]
pub use web_rpc::fetch_treasury_status;
