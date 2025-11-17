export interface TeamMember {
  id: string;
  name: string;
  role: string;
  bio: string;
  avatar: string;
  links: {
    email?: string;
    linkedin?: string;
    github?: string;
    website?: string;
    blog?: string;
  };
}
