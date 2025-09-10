import React from "react";

interface ToggleButtonsProps {
  options: string[];
  selected: string[];
  onChange: (selected: string[]) => void;
}

const ToggleButtons: React.FC<ToggleButtonsProps> = ({ options, selected, onChange }) => {
  const toggleOption = (option: string) => {
    if (selected.includes(option)) {
      onChange(selected.filter((o) => o !== option));
    } else {
      onChange([...selected, option]);
    }
  };
  
  return (
    <div className="flex gap-2 flex-wrap">
      {options.map((option) => {
        const isSelected = selected.includes(option);
        return (
  <button
    key={option}
    onClick={() => toggleOption(option)}
    className={`px-4 py-2 rounded transition-colors
      ${
        isSelected
          ? "bg-yellow-400 text-white border-3 border-yellow-500" // bright yellow highlight
          : "text-gray-600 border-gray-300 hover:bg-gray-100" // neutral default
      }`}
  >
    {option}
  </button>
);
      })}
    </div>
  );
};

export default ToggleButtons;
