const secondsInMinute = 60;
const secondsInHour = 3600;
const secondsInDay = 86400;

export const formatDateForSettingsList = (input: Date): string => {
  const now = new Date();
  const date = new Date(input);
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);


  if (diffInSeconds < secondsInMinute) {
    return 'now';
  } else if (diffInSeconds < secondsInHour) {
    const minutes = Math.floor(diffInSeconds / secondsInMinute);
    return `${minutes} minute${minutes !== 1 ? 's' : ''} ago`;
  } else if (diffInSeconds < secondsInDay) {
    const hours = Math.floor(diffInSeconds / secondsInHour);
    return `${hours} hour${hours !== 1 ? 's' : ''} ago`;
  } else {
    const yesterday = new Date(now);
    yesterday.setDate(now.getDate() - 1);

    if (
      date.getDate() === yesterday.getDate() &&
      date.getMonth() === yesterday.getMonth() &&
      date.getFullYear() === yesterday.getFullYear()
    ) {
      return 'yesterday';
    } else {
      return date.toLocaleDateString('en-US', {
        timeZone: 'UTC',
        year: 'numeric',
        month: 'short',
        day: 'numeric',
      });
    }
  }
};
