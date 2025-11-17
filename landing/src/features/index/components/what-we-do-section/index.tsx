import { motion } from 'framer-motion';
import { services, approachItems } from '../../data';
import { ArrowRight } from 'lucide-react';

export function WhatWeDoSection() {
  return (
    <section id="what-we-do" className="bg-slate-950 py-20 px-4">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-5xl font-bold text-white mb-6">
            What <span className="text-green-400">we do</span>
          </h2>
          <p className="text-slate-400 max-w-3xl mx-auto">
            At Biyard, we leverage advanced technologies like Blockchain,
            Artificial Intelligence, and Security to solve real-world problems.
            Our focus spans critical sectors including
          </p>
        </div>

        <div className="grid md:grid-cols-3 gap-8 mb-20">
          {services.map((service, index) => (
            <motion.div
              key={service.id}
              initial={{ opacity: 0, y: 50 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              viewport={{ once: true }}
              className="bg-slate-900 rounded-lg overflow-hidden border border-slate-800 hover:border-green-400 transition-all group"
            >
              <div className="aspect-video bg-slate-800 flex items-center justify-center">
                <span className="text-6xl">
                  {index === 0 ? '🎨' : index === 1 ? '🗳️' : '🌱'}
                </span>
              </div>
              <div className="p-6">
                <h3 className="text-xl font-bold text-white mb-3">
                  {service.title}
                </h3>
                <p className="text-slate-400 mb-4">{service.description}</p>
                {service.link && (
                  <a
                    href={service.link}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="inline-flex items-center text-green-400 hover:text-green-300 transition-colors"
                  >
                    Explore the service
                    <ArrowRight className="ml-2 w-4 h-4" />
                  </a>
                )}
              </div>
            </motion.div>
          ))}
        </div>

        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">
            + Our <span className="text-green-400">Approach</span>
          </h2>
        </div>

        <div className="grid md:grid-cols-3 gap-8">
          {approachItems.map((item, index) => (
            <motion.div
              key={item.id}
              initial={{ opacity: 0, y: 50 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              viewport={{ once: true }}
              className="bg-slate-900 p-8 rounded-lg border border-slate-800 text-center"
            >
              <div className="text-5xl mb-4">{item.icon}</div>
              <h3 className="text-2xl font-bold text-white mb-3">
                {item.title}
              </h3>
              <p className="text-slate-400">{item.description}</p>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
