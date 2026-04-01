import { Check } from "lucide-react";
import { Link } from "react-router-dom";
import { useState } from "react";

const tiers = [
  {
    name: "Free",
    price: "$0",
    period: "/month",
    description: "Perfect for getting started with Biyard.",
    features: [
      "1 Project",
      "1,000 API calls/month",
      "Community support",
      "Basic analytics",
      "100 active users",
    ],
    cta: "Get Started",
    ctaLink: "/sign-up",
    highlighted: false,
  },
  {
    name: "Growth",
    price: "$299",
    period: "/month",
    description: "For teams scaling their blockchain projects.",
    features: [
      "5 Projects",
      "50,000 API calls/month",
      "Priority email support",
      "Advanced analytics + Charts",
      "5,000 active users",
      "Custom challenges",
      "Webhook integrations",
    ],
    cta: "Get Started",
    ctaLink: "/sign-up?plan=growth",
    highlighted: true,
  },
  {
    name: "Enterprise",
    price: "Custom",
    period: "",
    description: "Tailored solutions for large-scale operations.",
    features: [
      "Unlimited Projects",
      "Unlimited API calls",
      "Dedicated account manager",
      "Custom SLA",
      "Unlimited users",
      "White-label SDK",
      "On-premise option",
      "Custom integrations",
    ],
    cta: "Contact Sales",
    ctaLink: "/contact",
    highlighted: false,
  },
];

const faqs = [
  {
    question: "Can I upgrade or downgrade anytime?",
    answer: "Yes, you can change plans at any time.",
  },
  {
    question: "What payment methods do you accept?",
    answer:
      "We accept credit cards, wire transfer, and crypto payments.",
  },
  {
    question: "Is there a free trial for Growth plan?",
    answer: "Yes, 14-day free trial with full features.",
  },
  {
    question: "Do you offer discounts for annual billing?",
    answer: "Yes, 20% discount for annual plans.",
  },
];

export function PricingPage() {
  const [openFaq, setOpenFaq] = useState<number | null>(null);

  return (
    <div className="max-w-6xl mx-auto px-4 py-12">
      {/* Header */}
      <div className="text-center mb-12">
        <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-4">
          Simple, Transparent Pricing
        </h1>
        <p className="text-lg text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
          Choose the plan that fits your project. Scale as you grow with
          flexible pricing designed for blockchain builders.
        </p>
      </div>

      {/* Pricing Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-20">
        {tiers.map((tier) => (
          <div
            key={tier.name}
            className={`relative rounded-2xl bg-white dark:bg-gray-800 shadow-lg p-8 flex flex-col ${
              tier.highlighted
                ? "border-2 border-blue-500 scale-105 z-10"
                : "border border-gray-200 dark:border-gray-700"
            }`}
          >
            {tier.highlighted && (
              <span className="absolute -top-3.5 left-1/2 -translate-x-1/2 bg-blue-500 text-white text-xs font-semibold px-4 py-1 rounded-full">
                Recommended
              </span>
            )}

            <h2 className="text-xl font-bold text-gray-900 dark:text-white">
              {tier.name}
            </h2>
            <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">
              {tier.description}
            </p>

            <div className="mt-6 mb-6">
              <span className="text-4xl font-extrabold text-gray-900 dark:text-white">
                {tier.price}
              </span>
              {tier.period && (
                <span className="text-gray-500 dark:text-gray-400 text-base">
                  {tier.period}
                </span>
              )}
            </div>

            <ul className="space-y-3 mb-8 flex-1">
              {tier.features.map((feature) => (
                <li
                  key={feature}
                  className="flex items-start gap-2 text-sm text-gray-700 dark:text-gray-300"
                >
                  <Check className="h-4 w-4 text-green-500 mt-0.5 shrink-0" />
                  <span>{feature}</span>
                </li>
              ))}
            </ul>

            <Link
              to={tier.ctaLink}
              className={`block text-center py-3 px-6 rounded-lg font-semibold text-sm transition-colors ${
                tier.highlighted
                  ? "bg-blue-500 text-white hover:bg-blue-600"
                  : "bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-gray-200 dark:hover:bg-gray-600"
              }`}
            >
              {tier.cta}
            </Link>
          </div>
        ))}
      </div>

      {/* FAQ Section */}
      <div className="max-w-3xl mx-auto">
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white text-center mb-8">
          Frequently Asked Questions
        </h2>
        <div className="space-y-4">
          {faqs.map((faq, idx) => (
            <div
              key={idx}
              className="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden"
            >
              <button
                onClick={() => setOpenFaq(openFaq === idx ? null : idx)}
                className="w-full flex items-center justify-between px-6 py-4 text-left bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-750 transition-colors"
              >
                <span className="font-medium text-gray-900 dark:text-white">
                  {faq.question}
                </span>
                <span className="text-gray-400 text-xl leading-none">
                  {openFaq === idx ? "\u2212" : "+"}
                </span>
              </button>
              {openFaq === idx && (
                <div className="px-6 pb-4 bg-white dark:bg-gray-800">
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    {faq.answer}
                  </p>
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
