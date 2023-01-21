import * as algolia from 'algoliasearch';
import { saveAs } from 'file-saver';
import * as React from 'react';
import {renderToString} from 'react-dom/server';

interface IURLSetProps {
  xmlns: string
}

declare global {
  namespace JSX {
    interface IntrinsicElements {
      url: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
      loc: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
      lastmod: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
      priority: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
      urlset: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement> & IURLSetProps, HTMLElement>;
    }
  }
}

interface Route {
  path: string;
  priority: number;
  params?: {
    [key: string]: string[]
  }
}

const routes: Route[] = [{
  path: '/',
  priority: 1
}, {
  path: '/experiences',
  priority: 0.5
}, {
  path: '/studies',
  priority: 0.4
}, {
  path: '/skills',
  priority: 0.7
}, {
  path: '/hobbies',
  priority: 0.5
}, {
  path: '/contact',
  priority: 0.5
}, {
  path: '/portfolio',
  priority: 0.5
}, {
  path: '/blog',
  priority: 1
}];

( window as any ).makeSitemap = async () => {
    try {
      const baseUrl = "https://www.cyprientaque.com";
      const now = new Date().toISOString();

      const instance = algolia('UGYL32VRFX', '51dc33de1cf4edee37afb0a67d9ad799');
      const index = instance.initIndex('projects_production');
      const browser = index.browseAll();
      const slugs: string[] = await new Promise((resolve, reject) => {
        browser.on('result', (result: algoliasearch.BrowseResponse) => {
          resolve(result.hits.filter((p: any) => p.category.slug === 'blog')
            .map((project: any) => project.slug));
        });
      });
      routes.push({
        path: '/blog/:slug',
        priority: 1,
        params: {
          slug: slugs
        }
      })
      const staticRoutes = routes
        .map((r: Route) => {
          return r.params
            ? Object.keys(r.params).map((param: string) =>
              (r.params as any)[param].map((value: string | number) => (
                <url key={r.path.replace(':' + param, value.toString())}>
                  <loc>{`${baseUrl}${r.path.replace(':' + param, value.toString())}`}</loc>
                  <lastmod>{now}</lastmod>
                  <priority>{r.priority}</priority>
                </url>
              )).reduce((accu: any[], item: any) => accu.concat(item), [])
            ).reduce((accu: any[], item: any) => accu.concat(item), [])
            :
            (
              <url key={r.path}>
                <loc>{`${baseUrl}${r.path}`}</loc>
                <lastmod>{now}</lastmod>
                <priority>{r.priority}</priority>
              </url>
            );
        }).reduce((accu: any[], item: any) => accu.concat(item), []);
        const xml = renderToString (
            <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
                {staticRoutes.map(( route : any ) => route)}
            </urlset>
        )
        const blob = new Blob([xml], {type: "text/xml;charset=utf-8"});
        saveAs(blob, "sitemap.xml");
    } catch (e) {
        console.error(e);
        return e;
    }
  }
