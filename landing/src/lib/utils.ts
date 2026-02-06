import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const scrollToSection = (id: string) => {
  const element = document.getElementById(id);
  if (element) {
    element.scrollIntoView({ behavior: "smooth" });
  }
};

export enum SectionIds {
  Top = "top",
  AboutUs = "about-us",
  Solution = "solution",
  Platforms = "platforms",

  HowItWorks = "how-it-works",
  Contact = "contact",
}
