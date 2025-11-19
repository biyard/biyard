import { ShapedArrowDown } from "@/components/icons";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Check } from "lucide-react";
import { useMemo, useState } from "react";

export interface TopicDropdownMenuProps {
  onChange: (key: string) => void;
  items: { key: string; label: string }[];
}

export function TopicDropdownMenu({ onChange, items }: TopicDropdownMenuProps) {
  const [selected, setSelected] = useState(0);

  const selectedTopic = useMemo(() => items[selected].label, [selected, items]);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <div className="flex flex-row gap-20 justify-end items-center px-20 w-full h-44 border-gray-600 cursor-pointer focus:outline-none rounded-[4px] border-b-1 focus:border-b-primary">
          <span>{selectedTopic}</span>
          <ShapedArrowDown
            className="[&>path]:stroke-gray-700 [&>path]:fill-gray-700 transition-all group-aria-expanded:rotate-180"
            width={20}
            height={20}
          />
        </div>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="bg-gray-900 border-primary" align="start">
        {items.map((topic, index) => (
          <DropdownMenuItem
            className="justify-between aria-selected:text-white"
            aria-selected={index === selected}
            onClick={() => {
              setSelected(index);
              onChange(topic.key);
            }}
          >
            <span>{topic.label}</span>
            {index === selected && <Check className="!w-15 !h-15" />}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
