import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { SettingsValueType, Settings } from "../types/settings";
import { createSetting } from "../api/settingsApi";

const CreateSettingsPage: React.FC = () => {
  const [key, setKey] = useState("");
  const [type, setType] = useState<SettingsValueType>(SettingsValueType.Str);
  const [value, setValue] = useState("");
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  const handleSave = async () => {
    setError(null);
    const newSetting: Settings = { key, type, value };

    try {
      await createSetting(newSetting);
      navigate("/settings");
    } catch (err) {
      setError(err instanceof Error ? err.message : "An unexpected error occurred");
    }
  };

  const isSaveDisabled = key.trim() === "" || value.trim() === "";
  return (
    <div className="max-w-5xl mx-auto p-6 bg-white rounded-lg shadow-md">
      {/* Top Bar: Back Button & Title */}
      <div className="flex justify-between items-center mb-4">
        <button
          onClick={() => navigate("/settings")}
          className="bg-gray-500 text-white px-4 py-2 rounded-md shadow hover:bg-gray-600 transition"
        >
          ← Back
        </button>
        <h2 className="text-2xl font-semibold">Create New Setting 🔧️</h2>
      </div>

      {/* Key Input */}
      <label className="block text-gray-700">Key:</label>
      <input
        type="text"
        value={key}
        onChange={(e) => setKey(e.target.value)}
        className="w-full p-2 border border-gray-300 rounded-md mb-4 focus:outline-none focus:ring-2 focus:ring-blue-400"
      />

      {/* Type Dropdown */}
      <label className="block text-gray-700">Type:</label>
      <select
        value={type}
        onChange={(e) => setType(e.target.value as SettingsValueType)}
        className="w-full p-2 border border-gray-300 rounded-md mb-4 focus:outline-none focus:ring-2 focus:ring-blue-400"
      >
        {Object.values(SettingsValueType).map((option) => (
          <option key={option} value={option}>
            {option}
          </option>
        ))}
      </select>

      {/* Value Input */}
      <label className="block text-gray-700">Value:</label>
      <textarea
        value={value}
        onChange={(e) => setValue(e.target.value)}
        rows={5}
        className="w-full p-2 border border-gray-300 rounded-md mb-4 focus:outline-none focus:ring-2 focus:ring-blue-400"
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
  );
};

export default CreateSettingsPage;