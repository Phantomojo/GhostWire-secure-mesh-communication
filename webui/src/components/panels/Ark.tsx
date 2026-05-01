import React, { useState, useEffect, useRef } from 'react';
import { useApp } from '../../context/AppContext';
import { ark as arkApi, type ArkEntry } from '../../services/api';

const ACCESS_OPTS = ['public','contacts','private'] as const;
const KIND_OPTS = ['note','document','link','data','key'] as const;

function EntryRow({ entry, active, onClick }: { entry: ArkEntry; active: boolean; onClick:()=>void }) {
  return (
    <button onClick={onClick} style={{
      display:'flex',flexDirection:'column',width:'100%',padding:'9px 12px',
      background:active?'var(--sig-ghost)':'none', border:`1px solid ${active?'var(--sig-dim)':'transparent'}`,
      borderRadius:'var(--r-sm)',cursor:'pointer',textAlign:'left',marginBottom:3,transition:'all 0.12s',
    }}>
      <div style={{ fontFamily:'var(--mono)',fontSize:11,fontWeight:active?700:500,color:active?'var(--sig)':'var(--t1)',overflow:'hidden',textOverflow:'ellipsis',whiteSpace:'nowrap' }}>
        {entry.title||'Untitled'}
      </div>
      <div style={{ display:'flex',gap:4,marginTop:3,flexWrap:'wrap' }}>
        <span className="ops-tag ops-tag-muted" style={{ fontSize:8 }}>{entry.kind}</span>
        <span className="ops-tag ops-tag-muted" style={{ fontSize:8 }}>{entry.collection||'default'}</span>
        {entry.access==='private' && <span className="ops-tag ops-tag-warn" style={{ fontSize:8 }}>private</span>}
      </div>
    </button>
  );
}

export default function ArkPanel() {
  const { toast } = useApp();
  const [entries, setEntries] = useState<ArkEntry[]>([]);
  const [collections, setCollections] = useState<string[]>([]);
  const [selected, setSelected] = useState<ArkEntry|null>(null);
  const [creating, setCreating] = useState(false);
  const [search, setSearch] = useState('');
  const [activeCol, setActiveCol] = useState<string|null>(null);
  const [loading, setLoading] = useState(false);
  const [stats, setStats] = useState<{total_entries:number;total_collections:number;total_tags:number}|null>(null);
  const searchTimeout = useRef<ReturnType<typeof setTimeout>>();

  // Form state
  const [form, setForm] = useState({ title:'', body:'', collection:'', kind:'note' as typeof KIND_OPTS[number], access:'private' as typeof ACCESS_OPTS[number], tags:'' });

  const loadAll = async () => {
    setLoading(true);
    try {
      const [cols, s] = await Promise.all([arkApi.collections(), arkApi.stats()]);
      setCollections(cols);
      setStats(s);
      if (search.trim()) {
        const results = await arkApi.search(search);
        setEntries(results.map(r => r.entry));
      } else if (activeCol) {
        const colEntries = await arkApi.byTag(activeCol).catch(() => []);
        setEntries(colEntries);
      } else {
        const results = await arkApi.search('').catch(() => []);
        setEntries(results.map(r => r.entry));
      }
    } catch { } finally { setLoading(false); }
  };

  useEffect(() => { loadAll(); }, [activeCol]);

  useEffect(() => {
    clearTimeout(searchTimeout.current);
    searchTimeout.current = setTimeout(() => {
      if (search.trim()) {
        arkApi.search(search).then(r => setEntries(r.map(e => e.entry))).catch(()=>{});
      } else { loadAll(); }
    }, 300);
  }, [search]);

  const handleCreate = async () => {
    if (!form.title.trim()) { toast('warn','Title required'); return; }
    setLoading(true);
    try {
      await arkApi.create({ title:form.title, body:form.body, collection:form.collection||'default', kind:form.kind, access:form.access, tags:form.tags.split(',').map(t=>t.trim()).filter(Boolean) });
      toast('success','Entry created');
      setCreating(false);
      setForm({ title:'',body:'',collection:'',kind:'note',access:'private',tags:'' });
      await loadAll();
    } catch (e:any) { toast('error',e.message??'Create failed'); }
    finally { setLoading(false); }
  };

  const handleDelete = async (cid: string) => {
    try { await arkApi.delete(cid); toast('warn','Entry deleted'); setSelected(null); await loadAll(); }
    catch (e:any) { toast('error',e.message??'Delete failed'); }
  };

  return (
    <div className="split-layout">
      {/* Left: collections */}
      <div className="split-sidebar" style={{ background:'var(--surface)',width:160,minWidth:140 }}>
        <div style={{ padding:'10px 10px 8px',borderBottom:'1px solid var(--border)' }}>
          <div className="ops-label" style={{ marginBottom:6 }}>Collections</div>
          {stats && <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)' }}>{stats.total_entries} entries</div>}
        </div>
        <div style={{ flex:1,overflowY:'auto',padding:4 }}>
          <button onClick={() => { setActiveCol(null); setSearch(''); }}
            style={{ width:'100%',padding:'7px 10px',background:activeCol===null?'var(--sig-ghost)':'none',
              border:`1px solid ${activeCol===null?'var(--sig-dim)':'transparent'}`,borderRadius:'var(--r-sm)',cursor:'pointer',
              fontFamily:'var(--mono)',fontSize:10,color:activeCol===null?'var(--sig)':'var(--t3)',textAlign:'left',marginBottom:2 }}>
            All entries
          </button>
          {collections.map(c => (
            <button key={c} onClick={() => setActiveCol(c)}
              style={{ width:'100%',padding:'7px 10px',background:activeCol===c?'var(--sig-ghost)':'none',
                border:`1px solid ${activeCol===c?'var(--sig-dim)':'transparent'}`,borderRadius:'var(--r-sm)',cursor:'pointer',
                fontFamily:'var(--mono)',fontSize:10,color:activeCol===c?'var(--sig)':'var(--t3)',textAlign:'left',marginBottom:2,
                overflow:'hidden',textOverflow:'ellipsis',whiteSpace:'nowrap' }}>
              {c}
            </button>
          ))}
        </div>
        <div style={{ padding:8,borderTop:'1px solid var(--border)' }}>
          <button className="btn-ops btn-ops-primary" onClick={() => { setCreating(true); setSelected(null); }} style={{ width:'100%',justifyContent:'center',fontSize:10 }}>
            + New entry
          </button>
        </div>
      </div>

      {/* Middle: entry list */}
      <div style={{ width:220,minWidth:180,borderRight:'1px solid var(--border)',display:'flex',flexDirection:'column',overflow:'hidden' }}>
        <div style={{ padding:'8px 10px',borderBottom:'1px solid var(--border)' }}>
          <input className="ops-input" placeholder="Search..." value={search} onChange={e => setSearch(e.target.value)} style={{ fontSize:11 }} />
        </div>
        <div style={{ flex:1,overflowY:'auto',padding:6 }}>
          {loading ? (
            <div style={{ padding:20,textAlign:'center',fontFamily:'var(--mono)',fontSize:10,color:'var(--t4)' }}>Loading...</div>
          ) : entries.length === 0 ? (
            <div style={{ padding:20,textAlign:'center',fontFamily:'var(--mono)',fontSize:10,color:'var(--t4)' }}>No entries</div>
          ) : entries.map(e => (
            <EntryRow key={e.cid} entry={e} active={selected?.cid===e.cid} onClick={() => { setSelected(e); setCreating(false); }} />
          ))}
        </div>
      </div>

      {/* Right: detail/create */}
      <div className="split-main" style={{ overflow:'auto' }}>
        {creating ? (
          <div style={{ padding:20 }}>
            <div className="ops-section-header" style={{ marginBottom:14 }}><span className="ops-section-title">New entry</span></div>
            {[
              { label:'Title', el:<input className="ops-input" value={form.title} onChange={e=>setForm({...form,title:e.target.value})} placeholder="Entry title" /> },
              { label:'Collection', el:<input className="ops-input" value={form.collection} onChange={e=>setForm({...form,collection:e.target.value})} placeholder="default" /> },
              { label:'Tags (comma-separated)', el:<input className="ops-input" value={form.tags} onChange={e=>setForm({...form,tags:e.target.value})} placeholder="tag1, tag2" /> },
            ].map(f => (
              <div key={f.label} style={{ marginBottom:10 }}>
                <div className="ops-label" style={{ marginBottom:5 }}>{f.label}</div>
                {f.el}
              </div>
            ))}
            <div style={{ display:'flex',gap:8,marginBottom:10 }}>
              <div style={{ flex:1 }}>
                <div className="ops-label" style={{ marginBottom:5 }}>Kind</div>
                <select className="ops-input" value={form.kind} onChange={e=>setForm({...form,kind:e.target.value as any})} style={{ width:'100%' }}>
                  {KIND_OPTS.map(k=><option key={k} value={k}>{k}</option>)}
                </select>
              </div>
              <div style={{ flex:1 }}>
                <div className="ops-label" style={{ marginBottom:5 }}>Access</div>
                <select className="ops-input" value={form.access} onChange={e=>setForm({...form,access:e.target.value as any})} style={{ width:'100%' }}>
                  {ACCESS_OPTS.map(a=><option key={a} value={a}>{a}</option>)}
                </select>
              </div>
            </div>
            <div style={{ marginBottom:14 }}>
              <div className="ops-label" style={{ marginBottom:5 }}>Content</div>
              <textarea className="ops-input" rows={10} value={form.body} onChange={e=>setForm({...form,body:e.target.value})}
                placeholder="Entry content..." style={{ resize:'vertical',width:'100%',fontFamily:'var(--mono)',fontSize:12 }} />
            </div>
            <div style={{ display:'flex',gap:8 }}>
              <button className="btn-ops btn-ops-primary" onClick={handleCreate} disabled={loading}>{loading?'Saving...':'⬟ Save entry'}</button>
              <button className="btn-ops" onClick={() => setCreating(false)}>Cancel</button>
            </div>
          </div>
        ) : selected ? (
          <div style={{ padding:20 }}>
            <div style={{ display:'flex',justifyContent:'space-between',alignItems:'flex-start',marginBottom:14,flexWrap:'wrap',gap:8 }}>
              <div>
                <div style={{ fontFamily:'var(--mono)',fontSize:16,fontWeight:700,color:'var(--t1)',marginBottom:6 }}>{selected.title}</div>
                <div className="tag-row">
                  <span className="ops-tag ops-tag-muted">{selected.kind}</span>
                  <span className="ops-tag ops-tag-muted">{selected.collection}</span>
                  <span className={`ops-tag ops-tag-${selected.access==='public'?'sig':selected.access==='contacts'?'info':'warn'}`}>{selected.access}</span>
                  {selected.tags?.map(t=><span key={t} className="ops-tag ops-tag-muted">{t}</span>)}
                </div>
              </div>
              <button className="btn-ops btn-ops-danger" onClick={() => handleDelete(selected.cid)} style={{ padding:'4px 10px',fontSize:10 }}>Delete</button>
            </div>
            <div style={{ fontFamily:'var(--mono)',fontSize:10,color:'var(--t4)',marginBottom:10 }}>
              CID: <span style={{ color:'var(--info)' }}>{selected.cid.slice(0,24)}...</span>
            </div>
            {selected.created_at && (
              <div style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t4)',marginBottom:14 }}>
                {new Date(selected.created_at*1000).toLocaleString()}
              </div>
            )}
            <div style={{ background:'var(--surface)',border:'1px solid var(--border)',borderRadius:'var(--r-sm)',padding:14,
              fontFamily:'var(--mono)',fontSize:12,color:'var(--t1)',whiteSpace:'pre-wrap',wordBreak:'break-word',lineHeight:1.6,
              maxHeight:400,overflowY:'auto' }}>
              {selected.body || <span style={{ color:'var(--t4)' }}>No content</span>}
            </div>
          </div>
        ) : (
          <div style={{ display:'flex',alignItems:'center',justifyContent:'center',height:'100%',flexDirection:'column',gap:8 }}>
            <div style={{ fontSize:32,color:'var(--t4)' }}>⬟</div>
            <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--t3)' }}>Select an entry or create one</div>
          </div>
        )}
      </div>
    </div>
  );
}
