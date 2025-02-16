const API_BASE_URL = import.meta.env.VITE_API_API_URL;

export const validateToken = async (token: string): Promise<void> => {
    const response = await fetch(`${API_BASE_URL}/auth/validate-token`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ token: token }),
    });

  if (!response.ok) {
    throw new Error("Invalid token");
  }
  return response.json();
};
