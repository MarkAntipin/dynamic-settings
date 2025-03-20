import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { SettingsValueType, CreateSettings } from "../types/settings";
import { createSetting } from "../api/settingsApi";
import KeyInput from "../components/KeyInput.tsx";
import TypeInput from '../components/TypeInput';
import PageHeader from "../components/PageHeader.tsx";
import ValueInput from "../components/ValueInput.tsx";

const CreateSettingsPage: React.FC = () => {
  const [key, setKey] = useState("");
  const [type, setType] = useState<SettingsValueType>(SettingsValueType.Str);
  const [value, setValue] = useState("");
  const [boolValue, setBoolValue] = useState(false);
  const [jsonError, setJsonError] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  // Reset value when type changes
  useEffect(() => {
    if (type === SettingsValueType.Bool) {
      setValue(boolValue ? "true" : "false");
    } else if (type === SettingsValueType.Json) {
      setValue("{}");
    } else if (type === SettingsValueType.Int) {
      setValue("0");
    } else if (type === SettingsValueType.Float) {
      setValue("0.0");
    } else {
      setValue("");
    }
  }, [type, boolValue]);

  const validateJson = (jsonStr: string) => {
    try {
      JSON.parse(jsonStr);
      setJsonError(null);
      return true;
    } catch (e) {
      setJsonError((e as Error).message);
      return false;
    }
  };

  const handleJsonChange = (newValue: string) => {
    setValue(newValue);
    validateJson(newValue);
  };

  const handleSave = async () => {
    setError(null);
    const newSetting: CreateSettings = { key, type, value };

    try {
      await createSetting(newSetting);
      navigate("/settings");
    } catch (err) {
      setError(err instanceof Error ? err.message : "An unexpected error occurred");
    }
  };

  const isSaveDisabled = key.trim() === "" || value.trim() === "" || (type === SettingsValueType.Json && jsonError !== null);
  return (
    <div className="max-w-5xl mx-auto p-6 bg-white rounded-lg shadow-md">
      {/* Top Bar: Back Button & Title */}
      <PageHeader
        title="Manage Setting"
        emoji="✏️"
      />

      <div className="space-y-4">
      {/* Key Input */}
      <KeyInput
        value={key}
        onChange={setKey}
      />

      {/* Type Dropdown */}
      <TypeInput
        value={type}
        onChange={setType}
      />

      {/* Value Input */}
      <ValueInput
        type={type}
        value={value}
        onChange={setValue}
        onBoolChange={setBoolValue}
        onJsonChange={handleJsonChange}
        jsonError={jsonError}
      />

      {/* Save Button */}
      <button
        onClick={handleSave}
        disabled={isSaveDisabled}
        className={`w-full px-4 py-2 rounded-md shadow transition ${
          isSaveDisabled
            ? "bg-gray-400 cursor-not-allowed"
            : "bg-green-500 text-white hover:bg-green-600"
        }`}
      >
        Save
      </button>

      {/* Display Backend Error */}
      {error && <p className="text-red-500 mb-4">{error}</p>}
    </div>
    </div>
  );
};

export default CreateSettingsPage;