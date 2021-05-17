import { _, addMessages, getLocaleFromNavigator, init } from 'svelte-i18n';

import enUS from '../i18n/en-US.json';
import zhCN from '../i18n/zh-CN.json';

export const locale = getLocaleFromNavigator();

addMessages('en-US', enUS);
addMessages('zh-CN', zhCN);

init({
  fallbackLocale: 'en-US',
  initialLocale: locale,
});

export { _ } from 'svelte-i18n';
