import * as path from 'path';
import * as fs from 'fs';
import { NextApiRequest, NextApiResponse } from 'next';
import { NFTStorage, Blob } from 'nft.storage';

export default async function createERC721Metadata(
  req: NextApiRequest,
  res: NextApiResponse
) {
  console.log('--- req.body: ', req.body)
  
  const { reward, score, scoreOutOf, sessionId } = (req.body ||
    {}) as Partial<GenerateSvgMarkupProps>;

  const isInputValid = [reward, score, scoreOutOf].every(
    x => typeof x === 'number' && x > 0
  );

  if (isInputValid === false) {
    res.status(400).json({ message: 'Invalid input '});

    return;
  }

  const svgMarkup = await generateSvgMarkup({
    reward,
    score,
    scoreOutOf,
    sessionId,
  });

  const { NFT_STORAGE_TOKEN } = process.env;
  const client = new NFTStorage({ token: NFT_STORAGE_TOKEN });

  const nftCid = await client.storeBlob(
    new Blob([svgMarkup]),
  );
  const metadata = createMetadata(nftCid);
  const metadataCid = await client.storeBlob(
    new Blob([JSON.stringify(metadata)]),
  );

  console.log('--- res.status: ', metadataCid)
  res.status(200).send(metadataCid);
}

interface GenerateSvgMarkupProps {
  reward: number;
  score: number;
  scoreOutOf: number;
  sessionId: string;
}

async function generateSvgMarkup({
  reward,
  score,
  scoreOutOf,
  sessionId,
}): Promise<string> {
  const rewardFormatted = Intl.NumberFormat('en-US', {
    currency: 'USD',
    style: 'currency',
  }).format(reward);

  const svgTemplateFileBuffer = await fs.promises.readFile(
    path.resolve('public', 'nft-template.svg')
  );

  const svgTemplate = svgTemplateFileBuffer.toString('utf8');
  const svgMarkup = svgTemplate
    .replace('{reward}', `${rewardFormatted}`)
    .replace('{score}', score)
    .replace('{scoreOutOf}', scoreOutOf)
    .replace('{sessionId}', sessionId)
    .replace('{timestamp}', Date.now().toString());

  return svgMarkup;
}

function createMetadata(cid: string) {
  return {
    title: 'Assets Metadata',
    type: 'object',
    properties: {
      name: {
        type: 'string',
        description: 'Wehub Reward Winner!',
      },
      description: {
        type: 'string',
        description: 'A unique NTF minted in unique conditions for a winner of the Wehub Reward lottery',
      },
      image: {
        type: 'string',
        description: `https://ipfs.io/ipfs/${cid}`,
      },
    },
  };
}
