import { Settings } from "../types/settings";

const API_BASE_URL = import.meta.env.VITE_API_API_URL;
const API_KEY = import.meta.env.VITE_API_KEY;

export const fetchSettings = async (): Promise<Settings[]> => {
  const response = await fetch(`${API_BASE_URL}/settings`, {
    method: "GET",
    headers: {
      "X-Api-Key": API_KEY,
    },
  });

  if (!response.ok) {
    throw new Error("Failed to fetch settings");
  }

  return response.json();
};

export const fetchSettingByKey = async (key: string): Promise<Settings> => {
  const response = await fetch(`${API_BASE_URL}/settings/${key}`, {
    method: "GET",
    headers: {
      "X-Api-Key": API_KEY,
    },
  });

  if (!response.ok) {
    throw new Error("Setting not found");
  }

  return response.json();
};

export const createSetting = async (setting: Settings): Promise<void> => {
  try {
    const response = await fetch(`${API_BASE_URL}/settings`, {
      method: "POST",
      headers: {
        "X-Api-Key": API_KEY,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(setting),
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => null); // Try parsing JSON
      const errorMessage = errorData?.message || `Failed to create setting (Status: ${response.status})`;
      throw new Error(errorMessage); // Correctly throwing the string error message
    }
  } catch (error) {
    throw new Error(error instanceof Error ? error.message : "An unexpected error occurred");
  }
};

export const deleteSettingByKey = async (key: string): Promise<void> => {
  const response = await fetch(`${API_BASE_URL}/settings`, {
    method: "DELETE",
    body: JSON.stringify({ keys: [key] }),
    headers: {
      "X-Api-Key": API_KEY,
      "Content-Type": "application/json",
    },
  });
  console.log(response);
  if (!response.ok) {
    throw new Error("Failed to delete setting");
  }
}
