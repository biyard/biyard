import { createContext, useContext, useState, type ReactNode } from "react";

export interface Brand {
  id: string;
  name: string;
  segment: string;
  status: string;
  treasury_usd: number;
  floor_price: number;
  user_count: number;
  total_supply: number;
  circulating_supply: number;
  share_percent: number;
  fx_rate: number;
}

interface BrandContextType {
  brands: Brand[];
  selectedBrand: Brand | null;
  selectBrand: (brandId: string | null) => void;
}

const BrandContext = createContext<BrandContextType | undefined>(undefined);

export function BrandProvider({ children }: { children: ReactNode }) {
  const brands: Brand[] = [
    {
      id: "le-mouton",
      name: "Le Mouton",
      segment: "Fashion",
      status: "ACTIVE",
      treasury_usd: 24500,
      floor_price: 0.0245,
      user_count: 1250,
      total_supply: 1000000,
      circulating_supply: 45000,
      share_percent: 3.0,
      fx_rate: 1200,
    },
    {
      id: "cafe-blossom",
      name: "Cafe Blossom",
      segment: "F&B",
      status: "ACTIVE",
      treasury_usd: 12300,
      floor_price: 0.0246,
      user_count: 850,
      total_supply: 500000,
      circulating_supply: 22000,
      share_percent: 2.5,
      fx_rate: 1200,
    },
    {
      id: "runpulse",
      name: "RunPulse",
      segment: "Sports Tech",
      status: "ACTIVE",
      treasury_usd: 35800,
      floor_price: 0.0179,
      user_count: 2100,
      total_supply: 2000000,
      circulating_supply: 68000,
      share_percent: 4.0,
      fx_rate: 1200,
    },
  ];

  const [selectedBrandId, setSelectedBrandId] = useState<string | null>(null);

  const selectedBrand = selectedBrandId
    ? brands.find((b) => b.id === selectedBrandId) || null
    : null;

  return (
    <BrandContext.Provider
      value={{ brands, selectedBrand, selectBrand: setSelectedBrandId }}
    >
      {children}
    </BrandContext.Provider>
  );
}

export function useBrands() {
  const context = useContext(BrandContext);
  if (!context) throw new Error("useBrands must be used within BrandProvider");
  return context;
}
