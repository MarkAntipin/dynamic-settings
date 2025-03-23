import { SettingsValueType } from "../types/settings";
import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-json";
import "ace-builds/src-noconflict/theme-monokai";

interface ValueInputProps {
  type: SettingsValueType;
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
  onBoolChange?: (value: boolean) => void;
  onJsonChange?: (value: string) => void;
  jsonError?: string | null;
}

const ValueInput: React.FC<ValueInputProps> = ({
  type,
  value,
  onChange,
  disabled = false,
  onBoolChange,
  onJsonChange,
  jsonError,
}) => {
  const renderInput = () => {
    switch (type) {
      case SettingsValueType.Bool:
        return (
          <div className="flex items-center space-x-4 mb-4">
            {["true", "false"].map((boolValue) => (
              <label key={boolValue} className="inline-flex items-center">
                <input
                  type="radio"
                  checked={value === boolValue}
                  onChange={() => {
                    onChange(boolValue);
                    onBoolChange?.(boolValue === "true");
                  }}
                  disabled={disabled}
                  className="form-radio h-5 w-5 text-blue-600 disabled:opacity-75 disabled:cursor-not-allowed"
                />
                <span className={`ml-2 ${disabled ? 'text-gray-500' : 'text-gray-700'}`}>
                  {boolValue.charAt(0).toUpperCase() + boolValue.slice(1)}
                </span>
              </label>
            ))}
          </div>
        );

      case SettingsValueType.Int:
      case SettingsValueType.Float:
        return (
          <input
            type="number"
            step={type === SettingsValueType.Float ? "0.1" : "1"}
            value={value}
            onChange={(e) => onChange(e.target.value)}
            disabled={disabled}
            className={`w-full p-2 border border-gray-300 rounded-md bg-gray-100 mb-4 ${
              disabled 
                ? 'cursor-not-allowed opacity-75' 
                : 'focus:outline-none focus:ring-2 focus:ring-blue-400'
            }`}
          />
        );

      case SettingsValueType.Json:
        return (
          <div className="mb-4">
            <AceEditor
              mode="json"
              theme="tomorrow"
              onChange={(newValue) => {
                  if (!disabled) {
                    onChange(newValue);
                    onJsonChange?.(newValue);
                  }
              }}
              value={value}
              readOnly={disabled}
              name="json-editor"
              editorProps={{ $blockScrolling: true }}
              setOptions={{
                tabSize: 4,
                showPrintMargin: false,
                highlightActiveLine: true,
                fontSize: 14,
                showLineNumbers: false
              }}
              width="100%"
              height="300px"
              className="border border-gray-300 rounded-md"
            />
            {jsonError && (
              <p className="text-red-500 mt-1 text-sm">{jsonError}</p>
            )}
          </div>
        );

      default:
        return (
          <textarea
            value={value}
            onChange={(e) => onChange(e.target.value)}
            disabled={disabled}
            rows={5}
            className={`w-full p-2 border border-gray-300 rounded-md mb-4 bg-gray-100 ${
              disabled 
                ? 'cursor-not-allowed opacity-75' 
                : 'focus:outline-none focus:ring-2 focus:ring-blue-400'
            }`}
          />
        );
    }
  };

  return (
    <div>
      <label className="block text-gray-700 mb-1">Value:</label>
      {renderInput()}
    </div>
  );
};

export default ValueInput;