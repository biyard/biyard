import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export function Updates() {
  const [email, setEmail] = useState("");

  const handleSubmit = async () => {};

  return (
    <div className="flex flex-col items-center px-28 w-full">
      <div
        id="get-updates"
        className="flex flex-col gap-24 py-40 w-full rounded-2xl border border-gray-800 max-w-wrapper px-118 bg-black/50 backdrop-blur-[5px] max-tablet:px-16 max-tablet:py-24"
      >
        <h2 className="font-semibold whitespace-pre-line text-[28px]/36 max-tablet:text-xl/34 max-tablet:whitespace-normal">
          Stay in the loop with our latest tech <br />
          breakthroughs and service updates!
        </h2>

        <div className="flex flex-row gap-24 w-full max-tablet:flex-col max-tablet:gap-48 max-tablet:items-center">
          <Input
            name="email"
            placeholder="Please enter your email address."
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />

          <Button variant={"secondary"} size="lg" onClick={handleSubmit}>
            Get Updates
          </Button>
        </div>
      </div>
    </div>
  );
}
