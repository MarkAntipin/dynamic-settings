export const GetBadgeColor = (type: string): string => {
  switch (type.toLowerCase()) {
    case "bool":
      return "bg-green-500";
    case "str":
      return "bg-blue-500";
    case "int":
      return "bg-purple-500";
    case "float":
      return "bg-indigo-500";
    case "json":
      return "bg-yellow-500 text-black";
    default:
      return "bg-gray-500";
  }
};
