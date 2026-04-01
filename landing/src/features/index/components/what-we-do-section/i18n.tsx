import { useTranslation } from "react-i18next";

import ServiceRatelImg from "/services/ratel.png?url";
import ServiceDagitImg from "/services/dagit.png?url";
import ServiceSustainabilityImg from "/services/sus.png?url";

export const WhatWeDoSection = {
  en: {
    title: 'What <span class="text-primary">we do</span>',
    description:
      "At Biyard, we leverage advanced technologies like Blockchain, Artificial Intelligence, and Security to solve real- world problems. Our focus spans critical sectors including",

    services: [
      {
        id: "ratel",
        title: "Democracy & Governance",
        description:
          "Strengthening democratic processes with transparent, secure digital solutions.",
        link: "https://ratel.foundation",
        image: ServiceRatelImg,
      },
      {
        id: "dagit",
        title: "Art & Culture",
        description:
          "Revolutionizing art management and marketplaces through decentralized platforms.",
        image: ServiceDagitImg,
      },
      {
        id: "sustainability",
        title: "Sustainability",
        description:
          "Ensuring our technology contributes positively to society and the planet.",
        image: ServiceSustainabilityImg,
      },
    ],

    approaches: {
      practical: {
        title: "Practical",
        description: "Real solutions for real problems.",
      },
      people_centric: {
        title: "People-Centric",
        description: "Technology built with and for communities.",
      },
      scalable: {
        title: "Scalable",
        description: "Global vision, local impact.",
      },
    },
  },
  ko: {
    title: 'What <span class="text-primary">we do</span>',
    description:
      "At Biyard, we leverage advanced technologies like Blockchain, Artificial Intelligence, and Security to solve real- world problems. Our focus spans critical sectors including",

    services: [
      {
        id: "ratel",
        title: "Democracy & Governance",
        description:
          "Strengthening democratic processes with transparent, secure digital solutions.",
        link: "https://ratel.foundation",
        image: ServiceRatelImg,
      },
      {
        id: "dagit",
        title: "Art & Culture",
        description:
          "Revolutionizing art management and marketplaces through decentralized platforms.",
        image: ServiceDagitImg,
      },
      {
        id: "sustainability",
        title: "Sustainability",
        description:
          "Ensuring our technology contributes positively to society and the planet.",
        image: ServiceSustainabilityImg,
      },
    ],

    approaches: {
      practical: {
        title: "Practical",
        description: "Real solutions for real problems.",
      },
      people_centric: {
        title: "People-Centric",
        description: "Technology built with and for communities.",
      },
      scalable: {
        title: "Scalable",
        description: "Global vision, local impact.",
      },
    },
  },
};

export interface ApproachI18n {
  title: string;
  description: string;
}

export interface ApproachesI18n {
  practical: ApproachI18n;
  people_centric: ApproachI18n;
  scalable: ApproachI18n;
}

export interface ServiceI18n {
  id: string;
  title: string;
  description: string;
  link?: string;
  image: string;
}

export interface WhatWeDoSectionI18n {
  title: string;
  description: string;
  services: ServiceI18n[];
  approaches: ApproachesI18n;
}

export function useWhatWeDoSectionI18n(): WhatWeDoSectionI18n {
  const { t } = useTranslation("WhatWeDoSection");

  return {
    title: t("title"),
    description: t("description"),
    services: t("services", { returnObjects: true }) as Array<ServiceI18n>,
    approaches: t("approaches", { returnObjects: true }) as ApproachesI18n,
  };
}
