import { motion } from 'framer-motion';
import { useContactForm } from '../../hooks/useContactForm';
import { ChevronDown } from 'lucide-react';
import { useState } from 'react';

export function ContactSection() {
  const { formData, handleChange, handleSubmit, isSubmitting, submitted } =
    useContactForm();
  const [showTopicDropdown, setShowTopicDropdown] = useState(false);

  const topics = [
    'General Inquiry',
    'Partnership',
    'Investment',
    'Technical Support',
    'Media Inquiry',
  ];

  return (
    <section id="contact" className="bg-slate-900 py-20 px-4">
      <div className="max-w-3xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-5xl font-bold text-white mb-4">
            Contact <span className="text-green-400">Us</span>
          </h2>
        </div>

        {submitted ? (
          <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            className="bg-green-400 bg-opacity-20 border border-green-400 rounded-lg p-8 text-center"
          >
            <h3 className="text-2xl font-bold text-green-400 mb-2">
              Thank you!
            </h3>
            <p className="text-slate-300">
              We've received your message and will get back to you soon.
            </p>
          </motion.div>
        ) : (
          <form onSubmit={handleSubmit} className="space-y-6">
            <div className="grid md:grid-cols-2 gap-6">
              <div>
                <label className="block text-slate-300 mb-2 text-sm">
                  First name
                </label>
                <input
                  type="text"
                  value={formData.firstName}
                  onChange={(e) => handleChange('firstName', e.target.value)}
                  className="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-green-400 transition-colors"
                  placeholder="Name"
                  required
                />
              </div>
              <div>
                <label className="block text-slate-300 mb-2 text-sm">
                  Last name
                </label>
                <input
                  type="text"
                  value={formData.lastName}
                  onChange={(e) => handleChange('lastName', e.target.value)}
                  className="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-green-400 transition-colors"
                  placeholder="Name"
                  required
                />
              </div>
            </div>

            <div>
              <label className="block text-slate-300 mb-2 text-sm">Email</label>
              <input
                type="email"
                value={formData.email}
                onChange={(e) => handleChange('email', e.target.value)}
                className="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-green-400 transition-colors"
                placeholder="Email"
                required
              />
            </div>

            <div>
              <label className="block text-slate-300 mb-2 text-sm">
                Company name
              </label>
              <input
                type="text"
                value={formData.company}
                onChange={(e) => handleChange('company', e.target.value)}
                className="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-green-400 transition-colors"
                placeholder="Name"
              />
            </div>

            <div className="relative">
              <label className="block text-slate-300 mb-2 text-sm">
                Which topic best fit your needs?
              </label>
              <button
                type="button"
                onClick={() => setShowTopicDropdown(!showTopicDropdown)}
                className="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-green-400 transition-colors flex items-center justify-between"
              >
                <span>{formData.topic}</span>
                <ChevronDown className="w-5 h-5" />
              </button>
              {showTopicDropdown && (
                <div className="absolute z-10 w-full mt-2 bg-slate-800 border border-slate-700 rounded-lg overflow-hidden">
                  {topics.map((topic) => (
                    <button
                      key={topic}
                      type="button"
                      onClick={() => {
                        handleChange('topic', topic);
                        setShowTopicDropdown(false);
                      }}
                      className="w-full px-4 py-3 text-left text-white hover:bg-slate-700 transition-colors"
                    >
                      {topic}
                    </button>
                  ))}
                </div>
              )}
            </div>

            <div>
              <label className="block text-slate-300 mb-2 text-sm">
                How can we help?
              </label>
              <textarea
                value={formData.message}
                onChange={(e) => handleChange('message', e.target.value)}
                className="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-green-400 transition-colors h-32 resize-none"
                placeholder="Please share what you want us to help"
                required
              />
            </div>

            <button
              type="submit"
              disabled={isSubmitting}
              className="w-full bg-green-400 hover:bg-green-500 text-slate-900 font-bold py-3 px-6 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isSubmitting ? 'Submitting...' : 'Submit'}
            </button>
          </form>
        )}
      </div>
    </section>
  );
}
