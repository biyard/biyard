import { Logo } from "@/components/icons";
import { useState } from "react";
import { Menu } from "lucide-react";
import { Button } from "@/components/button";
import { config } from "@/config";
import { scrollToSection, SectionIds } from "@/lib/utils";

export function Navigation() {
  const [menuOpen, setMenuOpen] = useState(false);
  const handleSectionClick = (id: string) => {
    scrollToSection(id);
    setMenuOpen(false);
  };

  return (
    <header className="flex fixed top-0 left-0 z-50 justify-center items-center w-screen">
      <div
        className="flex z-20 flex-row gap-10 justify-between items-center px-30 py-20 my-24 w-full rounded-2xl group
        max-w-desktop backdrop-blur-md bg-[rgba(44,40,55,0.5)] max-desktop:max-w-[calc(100vw-40px)]
        max-tablet:backdrop-blur-md max-tablet:bg-[rgba(0,0,0,0.5)] max-tablet:aria-expanded:bg-[rgba(0,0,0,0.95)]
        max-tablet:my-0 max-tablet:max-w-full max-tablet:aria-expanded:h-screen max-tablet:flex-col"
        aria-expanded={menuOpen}
      >
        <div className="flex flex-row justify-between w-full">
          <button
            onClick={() => handleSectionClick(SectionIds.Top)}
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
          <MenuItem onClick={() => handleSectionClick(SectionIds.AboutUs)}>
            About Us
          </MenuItem>
          <MenuItem onClick={() => handleSectionClick(SectionIds.Solution)}>
            Solution
          </MenuItem>
          <MenuItem onClick={() => handleSectionClick(SectionIds.Platforms)}>
            Case Study
          </MenuItem>
          <MenuItem onClick={() => handleSectionClick(SectionIds.HowItWorks)}>
            How it Works
          </MenuItem>
          <MenuItem onClick={() => handleSectionClick(SectionIds.Contact)}>
            Contact Us
          </MenuItem>

          <Button
            variant="primary"
            size="small"
            shape="pill"
            onClick={() => {
              window.open(config.consoleUrl, "_blank");
            }}
          >
            Go to Console
          </Button>
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
