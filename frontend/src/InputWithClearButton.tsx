import { X } from "lucide-react";

interface InputWithClearButtonProps  extends React.InputHTMLAttributes<HTMLInputElement> {
  value: string;  
  onClear: () => void;  
}

const InputWithClearButton = ({ value, onChange, onClear,...rest }: InputWithClearButtonProps) => {
  return (
    <div className="flex items-center border rounded-md overflow-hidden w-full max-w-md mb-2">
      <input
        type="text"
        value={value}
        onChange={onChange}
        className="flex-1 p-2 my-2 outline-none"        
        {...rest}
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
