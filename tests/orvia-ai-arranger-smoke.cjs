"use strict";
const assert=require("node:assert/strict");
const fs=require("node:fs");
const path=require("node:path");
const root=path.resolve(__dirname,"..");
const html=fs.readFileSync(path.join(root,"index.html"),"utf8");
const match=/<script>([\s\S]*?)<\/script>/.exec(html);
assert.ok(match,"inline application script");
const src=match[1].slice(0,match[1].indexOf('$("demo").onclick'));
new Function(src);
const nodes=new Map([
 ["chords",{value:"Am F C G"}],["bpm",{value:"76"}],["seed",{value:"2026"}],
 ["density",{value:"55"}],["energy",{value:"60"}],["melodySelect",{value:"0"}],
 ["bars",{}],["notes",{}],["export",{}],["status",{}]
]);
const document={getElementById(id){if(!nodes.has(id))throw Error("Unexpected DOM node: "+id);return nodes.get(id)}};
const tail="\n stop=()=>{};draw=()=>{};updateTracks=()=>{};sources=[{name:'Synthetic melody',notes:makeDemo(),program:0}]; return {api:{midiRead,trackEncode,parseChord,note,bytes,b16,b32,ppq,createArr},getResult:()=>result,getSource:()=>sources[0].notes};";
const factory=new Function("TextEncoder","TextDecoder","document",src+tail);
const app=factory(TextEncoder,TextDecoder,document),f=app.api;
for(const symbol of ["Am","F","C","G","D7","Cmaj7","Bm7","Dsus4"])assert.ok(f.parseChord(symbol).intervals.length>=3);
assert.equal(app.getSource().length,64);
f.createArr();
let tracks=app.getResult();
assert.equal(tracks.length,5);
assert.deepEqual(tracks[0].notes,app.getSource());
assert.equal(tracks[4].ch,9,"drum channel 10");
assert.ok(tracks.every(t=>t.notes.length>0));
const resultHash=JSON.stringify(tracks.map(t=>t.notes));
f.createArr();
assert.equal(JSON.stringify(app.getResult().map(t=>t.notes)),resultHash,"seed deterministic");
const midiTracks=[f.trackEncode({},76,true),...tracks.map(t=>f.trackEncode(t,76,false))];
let data=[...f.bytes("MThd"),...f.b32(6),...f.b16(1),...f.b16(midiTracks.length),...f.b16(f.ppq)];
for(const t of midiTracks)data.push(...t);
const parsed=f.midiRead(new Uint8Array(data));
assert.equal(parsed.tracks.length,5);
assert.equal(parsed.tempo,76);
assert.equal(parsed.tracks[0].notes.length,64);
// A Standard MIDI File quantizes note positions and lengths to integer PPQ ticks.
// Do not compare unquantized floating-point durations (e.g. 0.43 beats) exactly.
for (let i=0;i<tracks[0].notes.length;i++) {
 const got=parsed.tracks[0].notes[i],original=tracks[0].notes[i];
 assert.equal(got.p,original.p,"melody pitch "+i);
 assert.equal(got.v,original.v,"melody velocity "+i);
 assert.ok(Math.abs(got.t-original.t)<=1/f.ppq+1e-9,"melody start within 1 tick "+i);
 assert.ok(Math.abs(got.d-original.d)<=1/f.ppq+1e-9,"melody duration within 1 tick "+i);
}
assert.throws(()=>f.midiRead(new Uint8Array([0,1,2,3])),/Không phải tệp MIDI|MIDI bị cắt/);
console.log("PASS ORVIA AI Arranger V0 smoke: JS syntax, chords, four accompaniment roles, deterministic seed, MIDI SMF roundtrip, malformed input");
