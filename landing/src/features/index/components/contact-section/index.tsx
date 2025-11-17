import { motion } from "framer-motion";
import { useContactForm } from "../../hooks/useContactForm";
import { LabeledInput } from "@/components/ui/labeled_input";
import { Label } from "@/components/ui/label";
import { TopicDropdownMenu } from "./topic-dropdown-menu";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";

export function ContactSection() {
  const { formData, handleChange, handleSubmit, isSubmitting } =
    useContactForm();

  return (
    <section
      id="contact"
      className="flex relative flex-col justify-center mx-auto w-full max-w-wrapper"
    >
      <div className="absolute top-[1/2] left-[2/3] h-1328 w-1328 bg-purple-blur/40 blur-[500px] max-tablet:w-614 max-tablet:h-614 max-tablet:bg-purple-blur/80" />
      <div className="flex flex-row justify-between w-full">
        <div className="text-center">
          <h2 className="text-3xl font-bold text-white md:text-5xl">
            Contact <span className="text-green-400">Us</span>
          </h2>
        </div>

        <form onSubmit={handleSubmit} className="space-y-24">
          <div className="grid grid-cols-2 gap-24 max-tablet:grid-cols-1">
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

          <div className="flex flex-col">
            <Label>Which topic best fit your needs?</Label>
            <TopicDropdownMenu />
          </div>

          <div className="flex flex-col">
            <Label>How can we help?</Label>
            <Textarea
              value={formData.message}
              rows={4}
              onChange={(e) => handleChange("message", e.target.value)}
              placeholder="Please share what you want us to help"
              required
            />
          </div>

          <Button variant="secondary" type="submit" disabled={isSubmitting}>
            {isSubmitting ? "Submitting..." : "Submit"}
          </Button>
        </form>
      </div>
    </section>
  );
}
