import { call } from "@/lib/call";
import { logger } from "@/lib/logger";
import { useState } from "react";

interface ContactFormData {
  firstName: string;
  lastName: string;
  email: string;
  company: string;
  topic: string;
  message: string;
}

export function useContactForm() {
  const dropdownItems = [
    {
      key: "GENERAL_INQUIRY",
      label: "General Inquiry",
    },
    {
      key: "PARTNERSHIP",
      label: "Partnership",
    },
    {
      key: "SALES",
      label: "Sales",
    },
    {
      key: "INVESTMENT",
      label: "Investment",
    },
    {
      key: "TECHNICAL_SUPPORT",
      label: "Technical Support",
    },
    {
      key: "MEDIA_INQUIRY",
      label: "Media Inquiry",
    },
  ];

  const [formData, setFormData] = useState<ContactFormData>({
    firstName: "",
    lastName: "",
    email: "",
    company: "",
    topic: "GENERAL_INQUIRY",
    message: "",
  });

  const [isSubmitting, setIsSubmitting] = useState(false);
  const [submitted, setSubmitted] = useState(false);

  const handleChange = (field: keyof ContactFormData, value: string) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      logger.debug("Form submitted:", formData);
      setSubmitted(true);
      setFormData({
        firstName: "",
        lastName: "",
        email: "",
        company: "",
        topic: "General Inquiry",
        message: "",
      });

      await call("POST", "/web/contacts", {
        first_name: formData.firstName,
        last_name: formData.lastName,
        email: formData.email,
        company_name: formData.company,
        needs: formData.topic,
        help: formData.message,
      });
    } catch (error) {
      logger.error("Error submitting form:", error);
    } finally {
      setIsSubmitting(false);
    }
  };

  return {
    dropdownItems,
    formData,
    handleChange,
    handleSubmit,
    isSubmitting,
    submitted,
  };
}
