import type { ReactNode } from "react";

// Partner-defined. Set in your own integration; the SDK does not impose any
// branding of its own.
export const metadata = {
  title: "Acme Rewards — Biyard SDK Example",
  description:
    "Reference integration of @biyard/sdk + @biyard/widget for a partner brand.",
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body
        style={{
          fontFamily: "system-ui, -apple-system, sans-serif",
          margin: 0,
          padding: 0,
        }}
      >
        {children}
      </body>
    </html>
  );
}
