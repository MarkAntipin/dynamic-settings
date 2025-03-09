import { formatDateForSettingsList } from './FormatDate';

describe('formatDate', () => {
  it('should return "now" for dates less than a minute ago', () => {
    const now = new Date();
    expect(formatDateForSettingsList(now)).toBe('now');
  });

  it('should return "1 minute ago" for dates exactly one minute ago', () => {
    const oneMinuteAgo = new Date(Date.now() - 60 * 1000);
    expect(formatDateForSettingsList(oneMinuteAgo)).toBe('1 minute ago');
  });

  it('should return "x minutes ago" for dates less than an hour ago', () => {
    const tenMinutesAgo = new Date(Date.now() - 10 * 60 * 1000);
    expect(formatDateForSettingsList(tenMinutesAgo)).toBe('10 minutes ago');
  });

  it('should return "1 hour ago" for dates exactly one hour ago', () => {
    const oneHourAgo = new Date(Date.now() - 60 * 60 * 1000);
    expect(formatDateForSettingsList(oneHourAgo)).toBe('1 hour ago');
  });

  it('should return "x hours ago" for dates less than a day ago', () => {
    const fiveHoursAgo = new Date(Date.now() - 5 * 60 * 60 * 1000);
    expect(formatDateForSettingsList(fiveHoursAgo)).toBe('5 hours ago');
  });

  it('should return "yesterday" for dates exactly one day ago', () => {
    const yesterday = new Date();
    yesterday.setDate(yesterday.getDate() - 1);
    expect(formatDateForSettingsList(yesterday)).toBe('yesterday');
  });

  it('should return formatted date for dates more than a day ago', () => {
    const twoDaysAgo = new Date();
    twoDaysAgo.setDate(twoDaysAgo.getDate() - 2);
    const expectedDate = twoDaysAgo.toLocaleDateString('en-US', {
      timeZone: 'UTC',
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
    expect(formatDateForSettingsList(twoDaysAgo)).toBe(expectedDate);
  });
});