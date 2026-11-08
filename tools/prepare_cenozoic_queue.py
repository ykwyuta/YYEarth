"""Prepare resumable built-in ImageGen requests; this script never calls an image API."""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PERIODS = {'Paleogene', 'Neogene', 'Quaternary'}

ANATOMY = {
    'djarthia': 'Tiny primitive marsupial relative: pointed snout, small rounded ears, brown-grey fine fur, pale belly, five delicate grasping toes, long tapering lightly furred tail; not a kangaroo.',
    'thylacotinga': 'Small primitive omnivorous marsupial relative, pointed muzzle, rounded ears, five-toed feet and long tail. Body proportions are inferred; use a restrained small possum-like reconstruction.',
    'australonycteris': 'A small early true bat, furry torso, small pointed ears and membranous wings supported by long fingers; five digits per forelimb, no feathered wings.',
    'murgonemys': 'Soft-shelled freshwater turtle: flattened oval leathery shell, long extensible neck, narrow head with small snorkel-like nostrils, webbed feet with claws; not a domed tortoise or marine flippered turtle.',
    'kambara': 'Small semi-aquatic mekosuchine crocodile, broad robust snout, low scuted body, four short clawed legs and long powerful laterally flattened tail.',
    'platyplectrum-casca': 'Compact small early Australian frog: broad head, two short forelegs, two folded powerful hindlegs with long toes, smooth mottled earthy skin, no tail.',
    'alamitophis': 'A single slender extinct madtsoiid snake, scaled skin and elongated tapering body; no legs, no frill, no modern cobra hood. Full-body form is inferred from vertebrae.',
    'balbaroo': 'Small basal kangaroo relative, quadrupedal walking posture, long balancing tail, short forearms, strong hindlegs; not a modern upright red kangaroo.',
    'badjcinus': 'Small early thylacinid, long narrow muzzle, rounded ears, low slender four-legged body and long tapering tail, subdued brown fur; do not assume modern thylacine stripes.',
    'ngapakaldia': 'Sheep-sized primitive herbivorous marsupial: sturdy quadrupedal body, broad rounded head, small ears, heavy limbs and short tail; not a tapir with an elephant trunk.',
    'muramura': 'Primitive medium-sized herbivorous wynyardiid marsupial, stocky quadrupedal body, broad short muzzle, rounded ears and clawed feet; an interpretive koala-wombat-relative reconstruction.',
    'janjucetus': 'Small archaic toothed stem mysticete whale: broad robust short head with large eyes, clearly visible pointed teeth in both jaws, streamlined body, two pectoral flippers and horizontal tail flukes. No baleen plates, no legs.',
    'mammalodon': 'Small archaic toothed stem mysticete whale with broad rounded short rostrum, teeth, two pectoral flippers and horizontal tail flukes; no baleen filter curtains and no walking legs.',
    'crossvallia': 'Large early Paleocene Antarctic stem penguin: upright or horizontally diving seabird body, pointed long bill, flipper-like short wings, webbed feet, restrained countershaded plumage; no flying pose and no emperor-penguin orange patches.',
    'notiolofos': 'Medium-sized early Antarctic ungulate-like herbivore, sturdy quadruped with blunt browsing muzzle, small ears, short brown coat and compact hoof-like digits; no antlers, horn, trunk or giraffe neck. Whole-body proportions are inferred from teeth.',
    'antarctodolops': 'Small primitive polydolopid marsupial-relative mammal, gently rounded muzzle, small rounded ears, brown fur, grasping clawed feet and long tail; body reconstruction is inferred from dental fossils.',
    'antarctic-frog': 'One early Antarctic calyptocephalellid frog relative: broad flattened head, large mouth, robust smooth mottled brown-green body, short forelimbs and powerful folded hindlimbs with long toes. No tail.',
    'cucullaea-raea': 'One living thick-shelled fossil ark clam, robust rounded-triangular inflated bivalve shell with strong growth lines; both valves anatomically joined, subtle living mantle at the opening; not a fossil rock slab.',
    'struthiolarella': 'One living Eocene struthiolariid marine snail: tall stepped spiral shell with a flared outer lip, thick growth-ribbed cream-brown shell, soft muscular foot and two short tentacles; not a hermit crab.',
    'southern-nothofagus': 'One representative southern beech tree, branching woody trunk, dense crown of small simple serrated green leaves; no large magnolia blossoms. An interpretive Nothofagaceae group representative.',
    'southern-podocarps': 'One representative southern podocarp conifer tree, layered branching crown with narrow flattened green leaves; no pine-like long needles or showy flowers.',
    'southern-araucarians': 'One representative ancient araucarian conifer, straight trunk and distinct tiered whorls of radiating branches, dense short stiff green foliage, no flowers.',
    'southern-ferns': 'A single ground-growing fern clump with several arching finely divided pinnate green fronds and a few unfurling fiddleheads, no flowers, no tall tree trunk.',
    'australian-proteaceae': 'One representative Australian proteaceous shrub, woody branching stems, leathery elongated green leaves and a restrained cream flower cluster; not a European rose or tropical orchid.',
    'woolly-mammoth': 'Woolly mammoth: high shoulder dome, sloping back, long shaggy reddish-brown fur, small ears, muscular trunk and two very long curved ivory upper tusks; four columnar legs.',
    'deinotherium': 'Deinotherium: large elephant relative with a trunk and TWO downward-curving tusks emerging from the LOWER jaw; no upper tusks and no woolly fur.',
    'mastodon': 'American mastodon: stocky forest proboscidean, low rounded skull, long trunk, two forward-curving upper tusks, coarse brown coat; not a high-domed woolly mammoth.',
    'loxodonta': 'African elephant: large fan-shaped ears, long muscular trunk, sparse grey wrinkled skin, upper ivory tusks and four pillar-like legs.',
    'platybelodon': 'Platybelodon: elephant relative with a trunk and very long broad flattened lower jaw ending in a shovel of paired flattened LOWER tusks, plus two small upper tusks.',
    'gomphotherium': 'Gomphotherium: primitive elephant relative with a trunk, TWO upper tusks and TWO smaller forward lower tusks on an elongated lower jaw; not a woolly mammoth.',
    'arsinoitherium': 'Arsinoitherium: heavy quadrupedal herbivore with TWO enormous side-by-side conical horns on its nose and a smaller pair above its eyes; ears and heavy feet, no trunk.',
    'paraceratherium': 'Giant hornless rhino relative: long neck, long columnar legs, barrel chest and long browsing head with a short grasping upper lip. Absolutely no nasal horn, trunk, antlers or giraffe ossicones.',
    'uintatherium': 'Uintatherium: massive short-necked herbivore, THREE PAIRS of blunt bony knobs along skull and long downward upper saber canines, small ears, robust feet; no elephant trunk.',
    'embolotherium': 'Embolotherium: massive Asian brontothere with a broad upright battering-ram-shaped bony nasal plate/shield, stocky rhino-like body and four thick legs; not a single pointed rhino horn.',
    'elasmotherium': 'Elasmotherium: stocky giant rhinoceros with one large horn on the forehead, long low head, coarse dark fur and powerful grazing muzzle; horn shape is interpretive.',
    'woolly-rhino': 'Woolly rhinoceros: shaggy brown-grey body, low head with TWO nasal horns, long flattened front horn and smaller rear horn, short ears and robust legs.',
    'teleoceras': 'Teleoceras: short-legged barrel-bodied rhinoceros with a small nasal horn, broad low head and grey-brown wrinkled skin; no hippo teeth or water dependency implied.',
    'smilodon': 'Smilodon: powerfully built saber-toothed cat, very strong forequarters, SHORT bobtail and TWO long narrow downward upper canine teeth; no modern tiger stripe requirement.',
    'thylacoleo': 'Marsupial lion: stocky Australian marsupial predator with broad head, powerful clawed forelimbs, opposable large thumb claw and short blunt muzzle; large cutting premolars, NOT long feline saber canines.',
    'panthera-leo': 'African lion, adult lioness representative, short tawny fur, muscular quadrupedal cat body, rounded ears and long tufted tail; ordinary cat canines, no saber teeth.',
    'dire-wolf': 'Dire wolf: robust large canid, broad muzzle and heavy jaws, pointed ears, brown-grey coat, long bushy tail; not a saber-toothed cat or fantasy giant.',
    'megaloceros': 'Giant deer: tall deer body and exceptionally broad flattened palmate antlers with branching tines, long legs, short tail and brown coat.',
    'sivatherium': 'Sivatherium: robust short-necked giraffid, heavy shoulders, long legs, two large branching ossicones and a small front pair, no deer-like broad palmate antler paddles.',
    'giraffa': 'Giraffe: extremely long upright neck, long slender legs, small skin-covered paired ossicones and natural brown patch pattern, tail with a dark tuft.',
    'glyptodon': 'Glyptodon: low heavily armored armadillo relative with domed shell made of closely fitting bony mosaic scutes, short armored tail and squat clawed limbs; no turtle neck or club tail.',
    'diprotodon': 'Diprotodon: giant wombat-relative herbivorous marsupial, heavy barrel body, broad blunt head, large nasal opening, small rounded ears and sturdy clawed legs, no tusks or horns.',
    'macropus': 'Grey kangaroo: two powerful hind legs, short forearms, long heavy balancing tail, upright alert posture, grey-brown fur, narrow head with long ears; no pouch joey or extra animals.',
    'leptictidium': 'Leptictidium: small long-snouted early mammal, very short forearms, long hopping hindlegs and very long slender balancing tail; no kangaroo pouch.',
    'great-auk': 'Great auk, a flightless ALCID not a penguin: black back, white belly, thick black grooved bill, white oval patch near eye, short flipper-like wings and webbed feet.',
    'anthropornis': 'Very large Eocene stem penguin, long pointed bill, thick streamlined body, short stiff flipper-like wings, webbed feet, restrained countershading; no flying wings.',
    'pygoscelis': 'One brush-tailed penguin representative, black back and cap, white belly, short stiff flippers, webbed feet and short stiff tail; no spreading flight feathers.',
    'megadyptes': 'Yellow-eyed penguin representative, grey head with restrained pale yellow eye-band, white underside, dark back, short stiff flippers and webbed feet.',
    'presbyornis': 'Presbyornis: long thin wading legs, duck-like broad flattened bill, long neck, compact waterbird torso and folded feathered wings; not a flamingo curved bill.',
    'gastornis': 'Gastornis: giant heavy flightless plant-eating bird, huge deep blunt beak, thick legs and greatly reduced wings; no hooked raptor beak or flying pose.',
    'phorusrhacos': 'Phorusrhacos: tall flightless predatory terror bird, large deep hooked bill, long powerful running legs, reduced wings and compact tail; no flight pose.',
    'dromornis': 'Giant Australian flightless dromornithid, very heavy body, deep massive beak, stout legs and tiny wings; no feathered arms or flight pose.',
    'genyornis': 'Genyornis: large Australian flightless dromornithid, robust body, deep blunt bill, stout legs and tiny folded wings; no ostrich-like very thin neck.',
    'nummulites': 'One living large foraminifer, coin-shaped flat lenticular cream calcareous shell with subtle spiral growth pattern, no snail head, bivalve hinge or arthropod limbs.',
    'clypeaster': 'One living sea biscuit echinoid, low domed flattened oval test with FIVE petal-like ambulacral markings and very short velvety spines, no long porcupine spines or starfish arms.',
    'cenozoic-crabs': 'One representative true crab, broad oval carapace, TWO front pincers, FOUR pairs of walking legs, short eyestalks; no horseshoe-crab long sword tail.',
    'palaeocarpilius': 'One robust Eocene crab, smoothly rounded domed carapace, two strong front pincers, four pairs of walking legs and short eyestalks, no shrimp tail.',
    'bombus': 'One bumblebee, furry rounded yellow-and-black banded thorax and abdomen, SIX legs, TWO pairs of translucent insect wings and two antennae; no bird feathers.',
    'proplebeia': 'One small extinct stingless bee, dark brown lightly hairy body, SIX legs, TWO pairs of translucent wings, two antennae; no huge bumblebee stripes or amber block.',
    'obdurodon': 'An archaic toothed platypus relative: furry low body, broad flattened sensitive duck-like bill, four webbed clawed feet and a flat furry tail; teeth should not turn it into a crocodile.',
    'odobenocetops': 'A small walrus-faced toothed whale, rounded blunt head with a prominent long backward-directed upper tusk, streamlined body, two pectoral flippers and horizontal tail flukes; no walrus hind feet.',
    'hydrodamalis': 'Giant sea cow: huge rotund dark brown-grey wrinkled body, small rounded head with bristly muzzle, tiny foreflippers, no hindlegs and a broad horizontal tail fluke; not a seal or walrus.',
    'basilosaurus': 'Very elongated serpentine-bodied fully aquatic early whale, long tooth-bearing snout, small pectoral flippers, tiny non-weight-bearing hindlimb remnants and horizontal tail flukes; NOT a marine reptile.',
    'dorudon': 'Compact fully aquatic Eocene whale, long tooth-bearing snout, streamlined mammalian body, two pectoral flippers, tiny hindlimb remnants and horizontal tail flukes; no dorsal reptile crest.',
    'livyatan': 'Large raptorial sperm-whale relative with massive rectangular blunt head, functional large conical teeth in BOTH jaws, pectoral flippers and horizontal tail flukes; no baleen.',
    'megalodon': 'A gigantic robust lamniform shark, broad conical snout, five gill slits, triangular dorsal fin, large pectoral fins and vertical crescent tail; large broad serrated teeth, no mammal flippers.',
    'otodus-obliquus': 'A large Paleogene lamniform shark, pointed broad snout, five gill slits, triangular dorsal fin and vertical shark tail; prominent smooth-edged triangular teeth with small side cusplets.',
    'pakicetus': 'An early WALKING whale relative with a wolf-like long snout, small ears, four clearly visible weight-bearing legs, hoof-like toes and long tail. No flippers or whale tail flukes.',
    'desmostylus': 'A bulky semi-aquatic desmostylian mammal, large blunt head, short thick neck, four powerful weight-bearing legs with broad feet, small tail and short projecting incisors; no trunk or seal flippers.',
    'thalassocnus': 'An aquatic ground-sloth relative, elongated blunt head, heavy furry torso, four clawed limbs and sturdy tail; swimming close to the bottom, no dolphin fins or human face.',
    'chalicotherium': 'A large knuckle-walking chalicothere herbivore, horse-like head, long strong forelimbs longer than hindlimbs, huge curved claws, sloping back, no hoofed front feet.',
    'dryopithecus': 'An early European ape: no external tail, long arms, grasping hands and feet, small face and brown body hair; natural ape posture, not a modern gorilla caricature.',
    'proconsul': 'An early African ape, no external tail, quadrupedal arboreal proportions with grasping hands and feet, brown hair and relatively small head; not a modern human.',
    'aegyptopithecus': 'A small early catarrhine primate with a LONG TAIL, grasping hands and feet, moderately projecting muzzle and brown hair; not a tailless ape.',
    'plesiadapis': 'Primitive primate-relative mammal with long tail, prominent incisors, small rounded ears and clawed digits; not a modern monkey or lemur with large humanlike eyes.',
    'ardipithecus': 'One early hominin, small head, modest projecting muzzle, body hair, long arms and grasping divergent big toe, upright cautious stance; TWO arms and TWO legs, no tail. Nonsexual museum reconstruction with discreet opaque plant-fibre draping over the pelvis.',
    'australopithecus': 'One Australopithecus afarensis, small-brained early hominin with projecting muzzle, long arms, short stature and upright TWO-legged walk; no tail. Nonsexual museum reconstruction with discreet opaque plant-fibre draping over pelvis.',
    'homo-erectus': 'One Homo erectus adult, long legs, modern-like upright body proportions, low elongated skull and pronounced brow ridge, broad nose. TWO arms and TWO legs. Simple discreet unadorned hide wrap; no metal, modern clothing, jewelry or tools required.',
    'neanderthal': 'One adult Neanderthal, robust stocky cold-adapted body, short distal limbs, large nose, pronounced brow ridge and long low cranium, natural hide tunic and wrap, TWO arms and TWO legs; no modern objects.',
    'homo-sapiens': 'One early Homo sapiens adult, anatomically modern upright human body, high rounded skull and chin, simple modest unadorned hide clothing; TWO arms and TWO legs, no modern objects, no weapons required.',
}

def main():
    requests = json.loads((ROOT/'assets/requests.json').read_text(encoding='utf-8'))
    queue=[]
    for item in requests:
        if item['period'] not in PERIODS:
            continue
        for kind in ('artwork','unit'):
            if kind not in item or (ROOT/item[kind]['path']).exists():
                continue
            slug=item['slug']
            prompt=item[kind]['prompt']
            if kind=='unit':
                if slug in ('gastornis','phorusrhacos','dromornis','genyornis'):
                    prompt=prompt.replace('Flying with wings spread, clear side view, head pointing RIGHT and tail LEFT.', 'Flightless bird walking on two legs, wings folded, head facing RIGHT.')
                if slug in ('anthropornis','crossvallia','great-auk','pygoscelis','megadyptes'):
                    prompt=prompt.replace('Flying with wings spread, clear side view, head pointing RIGHT and tail LEFT.', 'Diving seabird swimming horizontally, flipper-like short wings, head facing RIGHT. Do not depict flight feathers or a flight pose.')
                if slug=='presbyornis':
                    prompt=prompt.replace('Flying with wings spread, clear side view, head pointing RIGHT and tail LEFT.', 'Standing wetland bird, wings folded, two long legs completely visible, bill points RIGHT.')
                if slug in ('homo-sapiens','homo-erectus','neanderthal','ardipithecus','australopithecus'):
                    prompt=prompt.replace('Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette.', 'One upright hominin walking on TWO legs, TWO arms, face points RIGHT; no tail. Entire body and both feet visible.')
            if slug in ANATOMY:
                prompt+=' Essential anatomy constraints: '+ANATOMY[slug]
            elif item['kind']=='species':
                prompt+=' Anatomically faithful to '+item['sci']+'. One complete representative organism, not a collage, chart, fossil skeleton, taxidermy mount or multiple views.'
            if item['kind']=='species' and kind=='artwork':
                prompt+=' Show one clear representative life form, no text or labels. Avoid additional distracting animals. Use a relevant habitat matching the subject, not an arbitrary modern savanna for Paleogene forest taxa.'
            if kind=='unit':
                prompt+=' Keep a completely clean alpha silhouette, no translucent background glow, no surrounding mist or ground shadow. Output background must genuinely be transparent.'
            queue.append({'slug':slug,'kind':kind,'path':item[kind]['path'],'prompt':prompt})
    # Start with the recent regional additions; then the other Cenozoic entries and environments.
    source=(ROOT/'src/life.rs').read_text(encoding='utf-8')
    focus=source.split('// Southern Paleogene regional coverage;',1)[1].split('/// 陸に降りて撮る景色',1)[0]
    order=re.findall(r'slug: "([^"]+)"',focus)
    queue.sort(key=lambda e:(0 if e['slug'] in order else 1, order.index(e['slug']) if e['slug'] in order else 0, 0 if e['kind']=='artwork' else 1))
    folder=ROOT/'output/cenozoic-generation'
    folder.mkdir(parents=True,exist_ok=True)
    (folder/'queue.json').write_text(json.dumps(queue,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(f'Prepared {len(queue)} built-in ImageGen requests.')

if __name__=='__main__':
    main()
