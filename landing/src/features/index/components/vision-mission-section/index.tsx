import { motion } from 'framer-motion';
import { coreValues } from '../../data';

export function VisionMissionSection() {
  return (
    <section id="intro" className="bg-slate-900 py-20 px-4">
      <div className="max-w-7xl mx-auto">
        <div className="grid md:grid-cols-2 gap-8 mb-20">
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="bg-slate-800 p-8 rounded-lg border border-slate-700"
          >
            <div className="flex items-center mb-4">
              <span className="text-4xl mr-4">👁️</span>
              <h3 className="text-2xl font-bold text-green-400">Vision</h3>
            </div>
            <p className="text-slate-300">
              To create a future where technology bridges gaps and transforms
              global challenges into sustainable opportunities.
            </p>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, x: 50 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="bg-slate-800 p-8 rounded-lg border border-slate-700"
          >
            <div className="flex items-center mb-4">
              <span className="text-4xl mr-4">🎯</span>
              <h3 className="text-2xl font-bold text-green-400">Mission</h3>
            </div>
            <p className="text-slate-300">
              Harness cutting-edge deep-tech innovations, including Blockchain,
              AI, and Security, to deliver practical solutions that empower
              communities, enhance transparency, and foster inclusive growth.
            </p>
          </motion.div>
        </div>

        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">
            Core <span className="text-green-400">Values</span>
          </h2>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
          {coreValues.map((value, index) => (
            <motion.div
              key={value.id}
              initial={{ opacity: 0, y: 50 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              viewport={{ once: true }}
              className="bg-slate-800 p-6 rounded-lg border border-slate-700 hover:border-green-400 transition-colors"
            >
              <div className="text-4xl mb-4">{value.icon}</div>
              <h4 className="text-xl font-bold text-white mb-2">
                {value.title}
              </h4>
              <p className="text-slate-400 text-sm">{value.description}</p>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
