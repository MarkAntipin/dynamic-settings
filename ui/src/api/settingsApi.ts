import {CreateSettings, Settings} from "../types/settings";

const API_BASE_URL = import.meta.env.VITE_API_API_URL;

const getAuthHeader = () => {
  const token = localStorage.getItem("authToken");
  if (!token) {
    throw new Error("Authentication token is missing");
  }

  return {
    "X-Api-Key": token,
    "Content-Type": "application/json",
  };
}

export const fetchSettings = async (): Promise<Settings[]> => {
  const response = await fetch(`${API_BASE_URL}/settings`, {
    method: "GET",
    headers: getAuthHeader(),
  });

  if (!response.ok) {
    throw new Error("Failed to fetch settings");
  }

  return response.json();
};

export const fetchSettingByKey = async (key: string): Promise<Settings> => {
  const response = await fetch(`${API_BASE_URL}/settings/${key}`, {
    method: "GET",
    headers: getAuthHeader(),
  });

  if (!response.ok) {
    throw new Error("Setting not found");
  }

  return response.json();
};

export const createSetting = async (setting: CreateSettings): Promise<void> => {
  try {
    const response = await fetch(`${API_BASE_URL}/settings`, {
      method: "POST",
      headers: getAuthHeader(),
      body: JSON.stringify(setting),
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => null);
      const errorMessage = errorData?.message || `Failed to create setting (Status: ${response.status})`;
      throw new Error(errorMessage);
    }
  } catch (error) {
    throw new Error(error instanceof Error ? error.message : "An unexpected error occurred");
  }
};

export const deleteSettingByKey = async (key: string): Promise<void> => {
  const response = await fetch(`${API_BASE_URL}/settings`, {
    method: "DELETE",
    body: JSON.stringify({ keys: [key] }),
    headers: getAuthHeader(),
  });
  if (!response.ok) {
    throw new Error("Failed to delete setting");
  }
}

export const updateSetting = async (key: string, value: string): Promise<void> => {
  const response = await fetch(`${API_BASE_URL}/settings`, {
    method: "PUT",
    body: JSON.stringify({ key: key,  value: value }),
    headers: getAuthHeader(),
  });
    if (!response.ok) {
      const errorData = await response.json().catch(() => null);
      const errorMessage = errorData?.message || `Failed to update setting (Status: ${response.status})`;
      throw new Error(errorMessage);
    }
}
