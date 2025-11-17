import { useNewsletter } from '../../hooks/useNewsletter';
import { Github, Linkedin } from 'lucide-react';

export function FooterSection() {
  const { email, setEmail, handleSubmit, isSubmitting, submitted } =
    useNewsletter();

  return (
    <footer className="bg-slate-950 py-16 px-4">
      <div className="max-w-7xl mx-auto">
        <div className="bg-slate-900 rounded-lg p-8 md:p-12 mb-8">
          <h2 className="text-2xl md:text-3xl font-bold text-white mb-6 text-center">
            Stay in the loop with our latest tech breakthroughs and service
            updates!
          </h2>
          <form onSubmit={handleSubmit} className="max-w-xl mx-auto">
            <div className="flex gap-3">
              <input
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="flex-1 bg-slate-800 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-green-400 transition-colors"
                placeholder="Please enter your email address."
                required
                disabled={isSubmitting}
              />
              <button
                type="submit"
                disabled={isSubmitting || submitted}
                className="bg-green-400 hover:bg-green-500 text-slate-900 font-bold px-6 py-3 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap"
              >
                {submitted ? 'Subscribed!' : isSubmitting ? 'Sending...' : 'Get Updates'}
              </button>
            </div>
          </form>
        </div>

        <div className="flex flex-col md:flex-row justify-between items-center gap-4 text-slate-400">
          <p>© Biyard. All rights reserved.</p>
          <div className="flex gap-4">
            <a
              href="https://github.com/biyard"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-green-400 transition-colors"
              aria-label="GitHub"
            >
              <Github className="w-6 h-6" />
            </a>
            <a
              href="https://www.linkedin.com/company/75498162"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-green-400 transition-colors"
              aria-label="LinkedIn"
            >
              <Linkedin className="w-6 h-6" />
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}
