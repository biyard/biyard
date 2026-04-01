import { useEffect, useRef } from "react";

export function HoverEffects() {
  const shadowRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const shadow = shadowRef.current;
    if (!shadow) return;

    const handleMouseMove = (event: MouseEvent) => {
      const target = event.target as HTMLElement | null;
      if (!target) return;

      const tagName = target.tagName.toLowerCase();
      const classList = target.className?.toString().split(/\s+/) ?? [];

      const isButton = tagName === "button";
      const isLink = tagName === "a";
      const hasHoverEffect = classList.includes("hover-effect");

      if (isButton || isLink || hasHoverEffect) {
        const x = event.pageX;
        const y = event.pageY;

        shadow.style.cssText = `
          left: ${x}px;
          top: ${y}px;
          position: absolute;
          width: 50px;
          height: 50px;
          background: radial-gradient(circle, rgba(100, 100, 100, 0.5), rgba(0, 0, 0, 0));
          border-radius: 50%;
          pointer-events: none;
          transform: translate(-50%, -50%);
          mix-blend-mode: screen;
          opacity: 0.7;
          filter: blur(15px);
          z-index: 9999;
        `;
      } else {
        shadow.style.cssText = "display: none;";
      }
    };

    document.addEventListener("mousemove", handleMouseMove);

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
    };
  }, []);

  return <div id="shadow" ref={shadowRef} style={{ display: "none" }} />;
}
