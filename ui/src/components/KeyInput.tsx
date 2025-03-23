interface KeyInputProps {
  value: string;
  onChange?: (value: string) => void;
  disabled?: boolean;
}

const KeyInput: React.FC<KeyInputProps> = ({ value, onChange, disabled = false }) => {
  return (
    <div>
      <label className="block text-gray-700 mb-1">Key:</label>
      <input
        type="text"
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        disabled={disabled}
        className={`w-full p-2 border border-gray-300 bg-gray-100 rounded-md ${
          disabled 
            ? 'cursor-not-allowed' 
            : 'focus:outline-none focus:ring-2 focus:ring-blue-400'
        }`}
      />
    </div>
  );
};

export default KeyInput;