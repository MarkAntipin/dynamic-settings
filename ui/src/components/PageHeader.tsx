import { useNavigate } from 'react-router-dom';

interface PageHeaderProps {
  title: string;
  emoji: string;
}

const PageHeader: React.FC<PageHeaderProps> = ({ title, emoji }) => {
  const navigate = useNavigate();

  return (
    <div className="flex justify-between items-center mb-4">
      <button
        onClick={() => navigate("/settings")}
        className="bg-gray-500 text-white px-4 py-2 rounded-md shadow hover:bg-gray-600 transition"
      >
        ← Back
      </button>
      <h2 className="text-2xl font-semibold">{title} {emoji}</h2>
    </div>
  );
};

export default PageHeader;
