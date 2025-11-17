import { motion } from 'framer-motion';
import { newsArticles } from '../../data';

export function PressSection() {
  const [featured, ...otherNews] = newsArticles;

  return (
    <section id="press-and-news" className="bg-slate-950 py-20 px-4">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-3xl md:text-5xl font-bold text-white mb-4">
            Press & <span className="text-green-400">News</span>
          </h2>
        </div>

        <div className="grid lg:grid-cols-2 gap-8">
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.6 }}
            viewport={{ once: true }}
            className="bg-slate-900 rounded-lg overflow-hidden border border-slate-800 hover:border-green-400 transition-all cursor-pointer group"
          >
            <div className="aspect-video bg-slate-800 flex items-center justify-center">
              <span className="text-6xl">📰</span>
            </div>
            <div className="p-6">
              <span className="inline-block px-3 py-1 bg-green-400 bg-opacity-20 text-green-400 text-xs rounded-full mb-3">
                {featured.category}
              </span>
              <h3 className="text-2xl font-bold text-white mb-3">
                {featured.title}
              </h3>
              <p className="text-slate-400">{featured.excerpt}</p>
            </div>
          </motion.div>

          <div className="grid gap-6">
            {otherNews.map((article, index) => (
              <motion.div
                key={article.id}
                initial={{ opacity: 0, x: 50 }}
                whileInView={{ opacity: 1, x: 0 }}
                transition={{ duration: 0.5, delay: index * 0.1 }}
                viewport={{ once: true }}
                className="bg-slate-900 rounded-lg overflow-hidden border border-slate-800 hover:border-green-400 transition-all cursor-pointer group flex"
              >
                <div className="w-32 h-32 bg-slate-800 flex items-center justify-center flex-shrink-0">
                  <span className="text-4xl">📄</span>
                </div>
                <div className="p-4 flex-1">
                  <span className="inline-block px-2 py-1 bg-green-400 bg-opacity-20 text-green-400 text-xs rounded-full mb-2">
                    {article.category}
                  </span>
                  <h4 className="text-lg font-bold text-white">
                    {article.title}
                  </h4>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}
