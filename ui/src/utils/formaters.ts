export const formatDate = (input: Date | string | number): string => {
  const date = new Date(input);

  return date.toLocaleDateString('en-US', {
    timeZone: 'UTC',
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
};
