import { ServiceCard } from "./service-card";
import { useWhatWeDoSectionI18n } from "./i18n";
import { Approach } from "./approach";
import { PeopleCentric, Practical, Scalable } from "@/components/icons";

export function WhatWeDoSection() {
  const t = useWhatWeDoSectionI18n();

  return (
    <section id="what-we-do" className="relative my-auto w-full">
      <div className="absolute left-[1/2] h-879 w-879 bg-purple-blur/50 blur-[300px]" />
      <div className="flex relative flex-col gap-48 justify-center items-center w-full min-h-screen max-w-wrapper">
        <div className="flex flex-col gap-24 items-center w-full z-1">
          <h1
            className="font-medium text-center text-white text-[45px]/64"
            dangerouslySetInnerHTML={{ __html: t.title }}
          />
          <p className="mx-auto max-w-3xl text-slate-400">{t.description}</p>
        </div>

        <div className="grid grid-cols-3 gap-25 max-tablet:grid-cols-1">
          {t.services.map((service) => (
            <ServiceCard
              key={service.id}
              title={service.title}
              description={service.description}
            >
              <img src={service.image} />
            </ServiceCard>
          ))}
        </div>

        <div className="flex flex-row justify-start items-center w-full gap-58 max-tablet:flex-col max-tablet:gap-32">
          <h2 className="font-medium text-left text-white text-[45px]/64 max-tablet:text-[32px]/42 max-tablet:w-full max-tablet:text-center w-263">
            + Our <br />
            <span className="text-primary">Approach</span>
          </h2>

          <div className="grid grid-cols-3 gap-64 justify-start items-start w-full max-tablet:grid-cols-1 max-tablet:gap-32">
            <Approach
              title={t.approaches.practical.title}
              description={t.approaches.practical.description}
            >
              <Practical />
            </Approach>
            <Approach
              title={t.approaches.people_centric.title}
              description={t.approaches.people_centric.description}
            >
              <PeopleCentric />
            </Approach>
            <Approach
              title={t.approaches.scalable.title}
              description={t.approaches.scalable.description}
            >
              <Scalable />
            </Approach>
          </div>
        </div>
      </div>
    </section>
  );
}
