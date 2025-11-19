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
        className="flex z-20 flex-row gap-10 justify-between items-center py-20 my-24 w-full rounded-2xl group max-w-1440 backdrop-blue-[50px] px-30 bg-menu-shade max-desktop:max-w-[calc(100vw-40px)] max-tablet:bg-transparent max-tablet:aria-expanded:bg-black max-tablet:my-0 max-tablet:max-w-full max-tablet:aria-expanded:h-screen max-tablet:flex-col"
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
          <MenuItem onClick={() => scrollToSection("intro")}>Intro</MenuItem>
          <MenuItem onClick={() => scrollToSection("what-we-do")}>
            What We Do
          </MenuItem>
          <MenuItem onClick={() => scrollToSection("our-team")}>
            Our Team
          </MenuItem>
          {/* <MenuItem onClick={() => scrollToSection("press-and-news")}>
            Press & News
          </MenuItem> */}
          <MenuItem onClick={() => scrollToSection("contact")}>
            Contact Us
          </MenuItem>
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
