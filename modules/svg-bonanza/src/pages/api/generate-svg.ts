import * as path from 'path';
import * as fs from 'fs/promises';
import { NextApiRequest, NextApiResponse } from 'next';

export default async function generateSvg(req: NextApiRequest, res: NextApiResponse) {
  const { reward, score, scoreOutOf, sessionId = 'skibidibiba' } = req.body || {};
  const rewardFormatted = Intl.NumberFormat('en-US', {
    currency: 'USD',
    style: 'currency'
  }).format(reward);

  const svgTemplateFileBuffer = await fs.readFile(
    path.resolve('src', 'assets', 'nft-template.svg')
  );

  const svgTemplate = svgTemplateFileBuffer.toString('utf8');
  const svgMarkup = svgTemplate
    .replace('{reward}', `${rewardFormatted}`)
    .replace('{score}', score)
    .replace('{scoreOutOf}', scoreOutOf)
    .replace('{signature}', sessionId);

  res.setHeader('Content-Type', 'image/svg+xml');
  res.status(200).send(svgMarkup);
}
