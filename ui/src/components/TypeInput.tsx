import { SettingsValueType } from "../types/settings";

interface TypeInputProps {
  value: string;
  onChange?: (value: SettingsValueType) => void;
  disabled?: boolean;
}
 // 🛒📝
const TypeInput: React.FC<TypeInputProps> = ({ value, onChange, disabled = false }) => {
  return (
    <div>
      <label className="block text-gray-700 mb-1">Type:</label>
      <select
        value={value}
        onChange={(e) => onChange?.(e.target.value as SettingsValueType)}
        disabled={disabled}
        className={`w-full p-2 border border-gray-300 bg-gray-100 rounded-md ${
          disabled 
            ? 'cursor-not-allowed' 
            : 'focus:outline-none focus:ring-2 focus:ring-blue-400'
        }`}
      >
        {Object.values(SettingsValueType).map((option) => (
          <option key={option} value={option}>
            {option}
          </option>
        ))}
      </select>
    </div>
  );
};

export default TypeInput;
