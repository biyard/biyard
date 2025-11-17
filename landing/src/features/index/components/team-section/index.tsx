import { motion } from 'framer-motion';
import { teamMembers } from '../../data';
import { Mail, Linkedin, Github, Globe } from 'lucide-react';

export function TeamSection() {
  return (
    <section id="our-team" className="bg-slate-900 py-20 px-4">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-3xl md:text-5xl font-bold text-white mb-4">
            Our <span className="text-green-400">Team</span>
          </h2>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
          {teamMembers.map((member, index) => (
            <motion.div
              key={member.id}
              initial={{ opacity: 0, y: 50 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: index * 0.05 }}
              viewport={{ once: true }}
              className="bg-slate-800 rounded-lg overflow-hidden border border-slate-700 hover:border-green-400 transition-all group"
            >
              <div className="aspect-square bg-gradient-to-br from-slate-700 to-slate-900 flex items-center justify-center">
                <span className="text-8xl">
                  {member.name.charAt(0).toUpperCase()}
                </span>
              </div>
              <div className="p-6">
                <div className="mb-4">
                  <p className="text-green-400 text-sm mb-1">{member.role}</p>
                  <h3 className="text-2xl font-bold text-white">
                    {member.name}
                  </h3>
                </div>

                <div className="flex gap-3 mb-4">
                  {member.links.email && (
                    <a
                      href={`mailto:${member.links.email}`}
                      className="text-slate-400 hover:text-green-400 transition-colors"
                      aria-label="Email"
                    >
                      <Mail className="w-5 h-5" />
                    </a>
                  )}
                  {member.links.linkedin && (
                    <a
                      href={member.links.linkedin}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-slate-400 hover:text-green-400 transition-colors"
                      aria-label="LinkedIn"
                    >
                      <Linkedin className="w-5 h-5" />
                    </a>
                  )}
                  {member.links.github && (
                    <a
                      href={member.links.github}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-slate-400 hover:text-green-400 transition-colors"
                      aria-label="GitHub"
                    >
                      <Github className="w-5 h-5" />
                    </a>
                  )}
                  {(member.links.website || member.links.blog) && (
                    <a
                      href={member.links.website || member.links.blog}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-slate-400 hover:text-green-400 transition-colors"
                      aria-label="Website"
                    >
                      <Globe className="w-5 h-5" />
                    </a>
                  )}
                </div>

                <p className="text-slate-400 text-sm line-clamp-4">
                  {member.bio}
                </p>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
