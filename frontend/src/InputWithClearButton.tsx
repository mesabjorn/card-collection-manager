import { X } from "lucide-react";

interface InputWithClearButtonProps {
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onClear: () => void;
}

const InputWithClearButton = ({ value, onChange, onClear }: InputWithClearButtonProps) => {
  return (
    <div className="flex items-center border rounded-md overflow-hidden w-full max-w-md">
      <input
        type="text"
        placeholder="Search by name"
        value={value}
        onChange={onChange}
        className="flex-1 p-2 outline-none"
      />
      {value && (
        <button
          onClick={onClear}
          className="p-2 text-gray-500 hover:text-red-600 transition-colors"
        >
          <X size={20} />
        </button>
      )}
    </div>
  );
};

export default InputWithClearButton;
