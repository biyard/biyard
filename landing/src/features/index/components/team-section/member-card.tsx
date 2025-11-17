import { Github, Linkedin, Mail } from "lucide-react";

function toColor(role: string): string {
  switch (role) {
    case "Founder & CEO":
      return "bg-member-ceo";
    case "CEO & CTO":
      return "bg-member-cto";
    case "Project Manager":
      return "bg-member-pm";
    case "UI/UX Designer":
      return "bg-member-designer";
    case "Software Engineer":
      return "bg-member-developer";
    case "Researcher":
      return "bg-member-researcher";
    case "Marketer":
      return "bg-member-marketer";
    default:
      return "bg-gray-500";
  }
}

export interface MemberSummary {
  role: string;
  name: string;
  image: string;
  email: string;
  web?: string;
  linkedin?: string;
  github?: string;
  bio: string;
}

export interface MemberCardProps {
  member: MemberSummary;
  className?: string;
  open?: boolean;
  onSelect?: () => void;
}

export function MemberCard({
  member,
  className,
  open = false,
  onSelect,
}: MemberCardProps) {
  const cardClass =
    `group w-full h-full overflow-hidden min-w-300 ${toColor(member.role)} ` +
    `rounded-[20px] overflow-hidden` +
    (className ? ` ${className}` : "");

  return (
    <div className={cardClass} onClick={() => onSelect?.()}>
      <div className="flex relative flex-col justify-center items-center bg-cover transition-all duration-1000 h-300 gap-[10px]">
        <div className="flex absolute top-0 left-0 flex-col justify-start items-center py-44 w-full">
          <img
            className="object-cover z-1"
            src={member.image}
            alt={member.name}
          />
        </div>

        <div className="member-card-bg z-2" />

        <div
          className={
            "absolute top-0 left-0 z-3 w-full h-full flex flex-col items-start " +
            "justify-end rounded-[8px] p-24 transition-all duration-1000 " +
            "group-hover:top-[100%] aria-open:top-[100%]"
          }
          aria-open={open}
        >
          <div className="font-medium text-opacity-30 transition-all duration-1000 text-lg/25">
            {member.role}
          </div>

          <div className="font-bold transition-all duration-1000 text-2xl/34">
            {member.name}
          </div>
        </div>

        <div
          className={
            "absolute transition-all duration-1000 top-[100%] left-0 w-full h-full " +
            "group-hover:top-0 aria-open:top-0 flex flex-col items-start " +
            "justify-start p-[20px] z-4 bg-black/85"
          }
          aria-open={open}
        >
          <div className="flex flex-col w-full">
            <div className="font-black text-[24px]">{member.role}</div>
            <div className="bg-white w-[68px] h-[5px]" />
            <div className="flex flex-row justify-between items-center w-full">
              <div className="font-black text-[20px]">{member.name}</div>
              <div className="flex flex-row justify-center items-center gap-[4px]">
                <a href={`mailto:${member.email}`}>
                  <Mail />
                </a>
                {member.linkedin && (
                  <a href={member.linkedin}>
                    <Linkedin />
                  </a>
                )}
                {member.github && (
                  <a href={member.github}>
                    <Github />
                  </a>
                )}
              </div>
            </div>
          </div>

          <div className="text-[14px] font-regular">{member.bio}</div>
        </div>
      </div>
    </div>
  );
}
