import { useTranslation } from "react-i18next";

export const Intro = {
  en: {
    vision: {
      title: "Vision",
      description:
        "To create a future where technology bridges gaps and transforms global challenges into sustainable opportunities.",
    },
    mission: {
      title: "Mission",
      description:
        "Harness cutting-edge deep-tech innovations, including Blockchain, AI, and Security, to deliver practical solutions that empower communities, enhance transparency, and foster inclusive growth.",
    },

    innovation: {
      title: "Innovation",
      description: "Continuously exploring and\npioneering new technologies",
    },

    inclusivity: {
      title: "Inclusivity",
      description: "Designing solutions accessible to\neveryone, everywhere.",
    },

    integrity: {
      title: "Integrity",
      description: "Upholding transparency and trust in\nevery interaction.",
    },

    sustainability: {
      title: "Sustainability",
      description:
        "Ensuring our technology contributes\npositively to society and the planet.",
    },
  },
  ko: {
    vision: {
      title: "Vision",
      description:
        "To create a future where technology bridges gaps and transforms global challenges into sustainable opportunities.",
    },
    mission: {
      title: "Mission",
      description:
        "Harness cutting-edge deep-tech innovations, including Blockchain, AI, and Security, to deliver practical solutions that empower communities, enhance transparency, and foster inclusive growth.",
    },

    innovation: {
      title: "Innovation",
      description: "Continuously exploring and\npioneering new technologies",
    },

    inclusivity: {
      title: "Inclusivity",
      description: "Designing solutions accessible to\neveryone, everywhere.",
    },

    integrity: {
      title: "Integrity",
      description: "Upholding transparency and trust in\nevery interaction.",
    },

    sustainability: {
      title: "sustainability",
      description:
        "Ensuring our technology contributes\npositively to society and the planet.",
    },
  },
};

export interface TitleDescriptionI18n {
  title: string;
  description: string;
}

export interface VisionMissionSectionI18n {
  vision: TitleDescriptionI18n;
  mission: TitleDescriptionI18n;
  innovation: TitleDescriptionI18n;
  inclusivity: TitleDescriptionI18n;
  integrity: TitleDescriptionI18n;
  sustainability: TitleDescriptionI18n;
}

export function useIntroI18n(): VisionMissionSectionI18n {
  const { t } = useTranslation("Intro");

  return {
    vision: t("vision", { returnObjects: true }) as TitleDescriptionI18n,
    mission: t("mission", { returnObjects: true }) as TitleDescriptionI18n,
    innovation: t("innovation", {
      returnObjects: true,
    }) as TitleDescriptionI18n,
    inclusivity: t("inclusivity", {
      returnObjects: true,
    }) as TitleDescriptionI18n,
    integrity: t("integrity", { returnObjects: true }) as TitleDescriptionI18n,
    sustainability: t("sustainability", {
      returnObjects: true,
    }) as TitleDescriptionI18n,
  };
}
