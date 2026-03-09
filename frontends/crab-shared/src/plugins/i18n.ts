import { FluentBundle, FluentResource } from "@fluent/bundle";
import { createFluentVue } from "fluent-vue";

import zhCNRaw from "../locales/zh-CN.ftl?raw";
import enUSRaw from "../locales/en-US.ftl?raw";

export const locales = ["zh-CN", "en-US"];

export function createI18n() {
    return createFluentVue({
        bundles: locales.map((locale) => {
            const bundle = new FluentBundle(locale);

            let resource;
            if (locale === "zh-CN") {
                resource = new FluentResource(zhCNRaw);
            } else {
                resource = new FluentResource(enUSRaw);
            }

            bundle.addResource(resource);
            return bundle;
        }),
    });
}
