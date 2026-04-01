import { Github, LinkedIn } from "@/components/icons";
export function FooterSection() {
  return (
    <div className="flex flex-col items-center w-full gap-393 max-tablet:gap-276">
      <footer className="flex relative flex-row justify-between items-center py-24 mx-auto w-full max-w-wrapper max-tablet:flex-col max-tablet:gap-24">
        <p className="order-1 font-extralight text-[15px]/23 max-tablet:order-2">
          © Biyard. All rights reserved.
        </p>

        <div className="flex flex-row order-2 gap-50 max-tablet:order-1">
          <a
            href="https://github.com/biyard"
            target="_blank"
            rel="noopener noreferrer"
            className="transition-colors hover:text-primary"
            aria-label="GitHub"
          >
            <Github />
          </a>
          <a
            href="https://www.linkedin.com/company/75498162"
            target="_blank"
            rel="noopener noreferrer"
            className="transition-colors hover:text-primary"
            aria-label="LinkedIn"
          >
            <LinkedIn />
          </a>
        </div>
      </footer>
    </div>
  );
}
