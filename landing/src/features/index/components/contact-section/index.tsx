import { useContactForm } from "../../hooks/useContactForm";
import { LabeledInput } from "@/components/ui/labeled_input";
import { Label } from "@/components/ui/label";
import { TopicDropdownMenu } from "./topic-dropdown-menu";
import { Textarea } from "@/components/ui/textarea";
import { Section } from "@/components/section";
import {
  SectionLabel,
  SectionTitle,
  BodyText,
  Highlight,
} from "@/components/typography";
import { Button } from "@/components/button";

const trustFeatures = [
  "On-chain audit trail for all treasury events.",
  "DAO-ready governance hooks for community oversight.",
  "Separation of approval, execution and monitoring.",
  "Designed for financial, EUA and payment partners.",
];

export function ContactSection() {
  const { formData, handleChange, handleSubmit, isSubmitting, dropdownItems } =
    useContactForm();

  return (
    <Section id="contact" className="bg-[#1A1D30]">
      <div className="flex flex-row w-full gap-80 max-tablet:flex-col max-tablet:gap-48">
        <div className="flex-1 flex flex-col gap-24">
          <SectionLabel className="text-xs">TRUST & COMPLIANCE</SectionLabel>
          <SectionTitle size="small" weight="black" align="left">
            <p>Institutional-grade transparency,</p>
            <Highlight>by default</Highlight>
          </SectionTitle>

          <BodyText size="small">
            Every credential, operation, and governance event is logged on-chain
            in tamper-proof, auditable format—so partners, regulators, and
            community members can verify your reserve discipline in real-time,
            just from an Etherscan (or equivalent) link.
          </BodyText>
          <ul className="flex flex-col gap-32 font-semibold text-sm mt-60">
            {trustFeatures.map((feature, index) => (
              <li
                key={index}
                className="flex items-center gap-12 text-gray-300 text-sm"
              >
                <div className="w-6 h-6 rotate-45 bg-primary"></div>
                <span>{feature}</span>
              </li>
            ))}
          </ul>
        </div>

        <div className="flex-1 flex flex-col gap-48 bg-white/5 p-32 rounded-3xl">
          <div className="flex flex-col gap-8">
            <SectionLabel className="text-xs">READY TO LAUNCH?</SectionLabel>
            <p className="text-white text-lg font-light">
              Let's design your revenue-backed launch model.
            </p>
          </div>

          <form onSubmit={handleSubmit} className="space-y-32 z-2 ">
            <div className="grid grid-cols-2 gap-24 max-tablet:grid-cols-1 ">
              <LabeledInput
                labelTitle="First name"
                type="text"
                value={formData.firstName}
                placeholder="Name"
                onChange={(e) => handleChange("firstName", e.target.value)}
                required
              />

              <LabeledInput
                labelTitle="Last name"
                type="text"
                value={formData.lastName}
                placeholder="Name"
                onChange={(e) => handleChange("lastName", e.target.value)}
                required
              />
            </div>

            <LabeledInput
              labelTitle="Email"
              type="email"
              value={formData.email}
              placeholder="Email"
              onChange={(e) => handleChange("email", e.target.value)}
            />

            <LabeledInput
              labelTitle="Company name"
              type="text"
              value={formData.company}
              placeholder="Name"
              onChange={(e) => handleChange("company", e.target.value)}
            />

            <div className="flex flex-col gap-8">
              <Label>Which topic best fit your needs?</Label>
              <TopicDropdownMenu
                items={dropdownItems}
                onChange={(key) => handleChange("topic", key)}
              />
            </div>

            <div className="flex flex-col gap-8">
              <Label>How can we help?</Label>
              <Textarea
                value={formData.message}
                rows={4}
                onChange={(e) => handleChange("message", e.target.value)}
                placeholder="Please share what you want us to help"
                required
              />
            </div>

            <Button size="large" shape="pill" fullWidth disabled={isSubmitting}>
              Book a Strategy Session
            </Button>

            <p className="text-gray-400 text-xs text-left">
              By submitting, you agree to be contacted about Biyard's enterprise
              solutions.
            </p>
          </form>
        </div>
      </div>
    </Section>
  );
}
