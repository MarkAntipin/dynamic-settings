import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { fetchSettings } from "../api/settingsApi";
import { Settings } from "../types/settings";

const ITEMS_PER_PAGE = 30;

const SettingsListPage: React.FC = () => {
  const [settings, setSettings] = useState<Settings[]>([]);
  const [filteredSettings, setFilteredSettings] = useState<Settings[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(1);
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    const getSettings = async () => {
      try {
        const data = await fetchSettings();
        setSettings(data);
        setFilteredSettings(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Unknown error");
      } finally {
        setLoading(false);
      }
    };
    getSettings();
  }, []);

  // Search filter logic
  useEffect(() => {
    const filtered = settings.filter(
      (setting) =>
        setting.key.toLowerCase().includes(searchQuery.toLowerCase())
    );
    setFilteredSettings(filtered);
    setCurrentPage(1); // Reset to first page on search
  }, [searchQuery, settings]);

  if (loading) return <p className="text-gray-600 text-center">Loading settings...</p>;
  if (error) return <p className="text-red-500 text-center">Error: {error}</p>;

  // Pagination logic
  const indexOfLastItem = currentPage * ITEMS_PER_PAGE;
  const indexOfFirstItem = indexOfLastItem - ITEMS_PER_PAGE;
  const currentSettings = filteredSettings.slice(indexOfFirstItem, indexOfLastItem);
  const totalPages = Math.ceil(filteredSettings.length / ITEMS_PER_PAGE);

  return (
    <div className="max-w-5xl mx-auto p-6 bg-white rounded-lg shadow-md">
      {/* Title */}
      <h2 className="text-2xl font-semibold mb-4">Dynamic Settings ⚙️</h2>

      {/* Search & Create Button */}
      <div className="flex justify-between items-center mb-4 gap-4">
        <input
          type="text"
          placeholder="Search by key..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full max-w-md p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-400"
        />
          <Link
            to="/settings/create"
            className="bg-green-500 text-white px-5 py-2 rounded-md shadow hover:bg-green-600 transition whitespace-nowrap"
          >
            ➕
          </Link>
      </div>

      {/* Settings Table */}
      <div className="overflow-x-auto">
        <table className="w-full border-collapse border border-gray-200">
          <thead>
            <tr className="bg-gray-100 text-left">
              <th className="p-2 border border-gray-200">Key</th>
              <th className="p-2 border border-gray-200">Type</th>
              <th className="p-2 border border-gray-200">Value</th>
            </tr>
          </thead>
          <tbody>
            {currentSettings.map((setting, index) => (
              <tr key={setting.key} className={index % 2 === 0 ? "bg-gray-50" : "bg-white"}>
                {/*<td className="p-2 border border-gray-200">{setting.key}</td>*/}

                <td className="p-2 border border-gray-200">
                  <Link to={`/settings/${setting.key}`} className="text-blue-600 hover:underline">
                    {setting.key}
                  </Link>
                </td>
                <td className="p-2 border border-gray-200">{setting.type}</td>
                <td className="p-2 border border-gray-200">
                    {setting.value.length > 50 ? `${setting.value.slice(0, 50)}...` : setting.value}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Pagination Controls */}
      <div className="flex justify-center items-center mt-4 space-x-2">
        <button
          onClick={() => setCurrentPage((prev) => Math.max(prev - 1, 1))}
          disabled={currentPage === 1}
          className="px-3 py-1 border rounded-md bg-gray-200 hover:bg-gray-300 disabled:opacity-50"
        >
          Prev
        </button>
        <span className="px-3 py-1 border rounded-md bg-gray-100">
          Page {currentPage} of {totalPages}
        </span>
        <button
          onClick={() => setCurrentPage((prev) => Math.min(prev + 1, totalPages))}
          disabled={currentPage === totalPages}
          className="px-3 py-1 border rounded-md bg-gray-200 hover:bg-gray-300 disabled:opacity-50"
        >
          Next
        </button>
      </div>
    </div>
  );
};

export default SettingsListPage;
