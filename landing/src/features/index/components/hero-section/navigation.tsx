import { Logo } from "@/components/icons";
import { useState } from "react";
import { Menu } from "lucide-react";

export function Navigation() {
  const [menuOpen, setMenuOpen] = useState(false);
  const scrollToSection = (id: string) => {
    const element = document.getElementById(id);
    if (element) {
      element.scrollIntoView({ behavior: "smooth" });
    }
    setMenuOpen(false);
  };

  return (
    <header className="flex fixed top-0 left-0 z-50 justify-center items-center w-screen">
      <div
        className="flex z-20 flex-row gap-10 justify-between items-center px-30 py-20 my-24 w-full rounded-2xl group max-w-desktop backdrop-blue-[50px] bg-[rgba(44,40,55,0.5)] max-desktop:max-w-[calc(100vw-40px)] max-tablet:bg-transparent max-tablet:aria-expanded:bg-black max-tablet:my-0 max-tablet:max-w-full max-tablet:aria-expanded:h-screen max-tablet:flex-col"
        aria-expanded={menuOpen}
      >
        <div className="flex flex-row justify-between w-full">
          <button
            onClick={() => scrollToSection("top")}
            className="flex items-center space-x-2"
          >
            <Logo />
          </button>
          <Menu
            className="hidden max-tablet:block"
            onClick={() => setMenuOpen(!menuOpen)}
          />
        </div>

        <nav className="flex flex-row gap-48 justify-center items-center font-semibold text-center font-outfit text-base/16 tracking-[0.5px] max-tablet:h-full max-tablet:flex-col max-tablet:z-100 max-tablet:bg-black max-tablet:hidden max-tablet:group-aria-expanded:flex">
          <MenuItem onClick={() => scrollToSection("about-us")}>
            About Us
          </MenuItem>
          <MenuItem onClick={() => scrollToSection("solution")}>
            Solution
          </MenuItem>
          <MenuItem onClick={() => scrollToSection("case-study")}>
            Case Study
          </MenuItem>
          <MenuItem onClick={() => scrollToSection("how-it-works")}>
            How it Works
          </MenuItem>
          <MenuItem onClick={() => scrollToSection("contact")}>
            Contact Us
          </MenuItem>

          <button className="flex flex-row justify-center items-center px-20 py-10 gap-10 font-semibold rounded-[50px] text-black hover:opacity-90 transition-opacity max-tablet:hidden bg-[linear-gradient(93.06deg,#00D190_0%,#A9B5F3_99.39%)] shadow-[0px_20px_50px_20px_rgba(0,230,165,0.15)] whitespace-nowrap">
            Go to Console
          </button>
        </nav>
      </div>
    </header>
  );
}

function MenuItem({
  onClick,
  children,
}: {
  onClick: () => void;
  children: React.ReactNode;
}) {
  const [hover, setHover] = useState(false);

  return (
    <button
      className="flex flex-col gap-7 items-center mt-7 min-w-110 max-tablet:hover:text-primary"
      onMouseEnter={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
      onClick={() => onClick()}
    >
      {children}
      <div
        className="w-0 h-1 transition-all duration-500 aria-hover:bg-primary aria-hover:w-full max-tablet:aria-hover:bg-transparent"
        aria-hover={hover}
      />
    </button>
  );
}
