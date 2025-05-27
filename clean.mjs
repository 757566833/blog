import  {
  existsSync, readdirSync, lstatSync, unlinkSync, rmdirSync,
}  from 'fs';
import { join }  from 'path';

import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const pwd = __dirname;

const deleteFolderRecursive = (folderPath) => {
  if (existsSync(folderPath)) {
    if (lstatSync(folderPath).isDirectory()) {
      readdirSync(folderPath).forEach((file) => {
        const curPath = join(folderPath, file);

        if (lstatSync(curPath).isDirectory()) {
          // 递归删除子文件夹
          deleteFolderRecursive(curPath);
        } else {
          // 删除文件
          unlinkSync(curPath);
        }
      });

      // 删除空文件夹
      rmdirSync(folderPath);
    } else {
      // 删除文件
      unlinkSync(folderPath);
    }
  }
};

const cache = [

  join(pwd, './node_modules'),
  join(pwd, './.turbo'),

  join(pwd, './apps/web/.next'),
  join(pwd, './apps/web/.turbo'),
  join(pwd, './apps/web/node_modules'),

  join(pwd, './packages/ui/.turbo'),
  join(pwd, './packages/ui/node_modules'),

  join(pwd, './packages/eslint-config/.turbo'),
  join(pwd, './packages/eslint-config/node_modules'),


];
for (let index = 0; index < cache.length; index += 1) {
  const element = cache[index];
  deleteFolderRecursive(element);
}
