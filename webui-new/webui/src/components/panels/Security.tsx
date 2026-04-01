import React, { useState, useEffect } from 'react';
import { useApp } from '../../context/AppContext';
import { security as secApi } from '../../services/api';

function ConfirmAction({ message, onConfirm, onCancel }: { message:string; onConfirm:()=>void; onCancel:()=>void }) {
  const [val, setVal] = useState('');
  return (
    <div className="inline-confirm">
      <div style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--crit)',marginBottom:8 }}>{message}</div>
      <div style={{ fontFamily:'var(--mono)',fontSize:10,color:'var(--t3)',marginBottom:6 }}>Type CONFIRM to proceed:</div>
      <div style={{ display:'flex',gap:6 }}>
        <input className="ops-input" value={val} onChange={e=>setVal(e.target.value)} placeholder="CONFIRM"
          style={{ flex:1,fontSize:11 }} autoFocus />
        <button className="btn-ops btn-ops-danger" onClick={onConfirm} disabled={val!=='CONFIRM'} style={{ padding:'4px 10px',fontSize:11 }}>Execute</button>
        <button className="btn-ops" onClick={onCancel} style={{ padding:'4px 10px',fontSize:11 }}>Cancel</button>
      </div>
    </div>
  );
}

export default function SecurityPanel() {
  const { state, refreshMesh, toast } = useApp();
  const [auditLog, setAuditLog] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [confirm, setConfirm] = useState<null|'lockdown'|'rotate'>(null);
  const [locking, setLocking] = useState(false);

  useEffect(() => {
    secApi.audit().then(setAuditLog).catch(() => {});
  }, []);

  const handleRotate = async () => {
    setLocking(true);
    try { await secApi.rotateKeys(); toast('success','Keys rotated — all sessions re-established'); setConfirm(null); }
    catch (e:any) { toast('error', e.message ?? 'Key rotation failed'); }
    finally { setLocking(false); }
  };

  const handleLockdown = async () => {
    setLocking(true);
    try { await secApi.lockdown(true); toast('warn','LOCKDOWN ACTIVE — all connections blocked'); setConfirm(null); }
    catch (e:any) { toast('error', e.message ?? 'Lockdown failed'); }
    finally { setLocking(false); refreshMesh(); }
  };

  const scoreColor = state.securityScore >= 80 ? 'var(--sig)' : state.securityScore >= 60 ? 'var(--warn)' : 'var(--crit)';
  const threatColor: Record<string,string> = { none:'var(--sig)',low:'var(--sig)',medium:'var(--warn)',high:'var(--crit)',critical:'var(--crit)' };

  return (
    <div className="panel-scroll animate-in">
      {/* Score hero */}
      <div className="ops-panel" style={{ padding:20,marginBottom:14,display:'flex',gap:24,flexWrap:'wrap',alignItems:'center' }}>
        <div>
          <div className="ops-label" style={{ marginBottom:4 }}>Security score</div>
          <div style={{ fontFamily:'var(--mono)',fontSize:52,fontWeight:700,color:scoreColor,lineHeight:1 }}>{state.securityScore}</div>
          <div style={{ fontFamily:'var(--mono)',fontSize:10,color:'var(--t3)',marginTop:4 }}>/100</div>
        </div>
        <div style={{ flex:1,minWidth:160 }}>
          <div style={{ marginBottom:10 }}>
            <div className="ops-label" style={{ marginBottom:4 }}>Threat level</div>
            <div className={`ops-tag ops-tag-${state.threatLevel==='none'||state.threatLevel==='low'?'sig':state.threatLevel==='medium'?'warn':'crit'}`} style={{ fontSize:11 }}>
              {(state.threatLevel||'none').toUpperCase()}
            </div>
          </div>
          <div>
            <div className="ops-label" style={{ marginBottom:4 }}>PQC armor</div>
            <div className={`ops-tag ops-tag-${state.pqcEnabled?'sig':'muted'}`}>{state.pqcEnabled?'ACTIVE':'INACTIVE'}</div>
          </div>
        </div>

        {/* Actions */}
        <div style={{ display:'flex',flexDirection:'column',gap:6 }}>
          <button className="btn-ops btn-ops-warn" onClick={() => setConfirm('rotate')} disabled={locking}>⟳ Rotate keys</button>
          <button className="btn-ops btn-ops-danger" onClick={() => setConfirm('lockdown')} disabled={locking}>⬢ Full lockdown</button>
        </div>
      </div>

      {/* Confirm dialogs */}
      {confirm === 'rotate' && (
        <div style={{ marginBottom:14 }}>
          <ConfirmAction
            message="Rotate all session keys? All active sessions will be re-established."
            onConfirm={handleRotate}
            onCancel={() => setConfirm(null)}
          />
        </div>
      )}
      {confirm === 'lockdown' && (
        <div style={{ marginBottom:14 }}>
          <ConfirmAction
            message="Enable lockdown? All peer connections will be blocked immediately."
            onConfirm={handleLockdown}
            onCancel={() => setConfirm(null)}
          />
        </div>
      )}

      {/* Stats grid */}
      {state.meshHealth && (
        <div className="metric-grid" style={{ marginBottom:14 }}>
          {[
            { label:'Messages encrypted', value:state.meshHealth.storage.message_count, accent:'var(--sig)' },
            { label:'Blocked peers', value:state.meshHealth.security.blocked_peers, accent:state.meshHealth.security.blocked_peers>0?'var(--warn)':'var(--t1)' },
            { label:'Failed unlocks 24h', value:state.meshHealth.security.failed_unlock_attempts_24h, accent:state.meshHealth.security.failed_unlock_attempts_24h>0?'var(--crit)':'var(--t1)' },
            { label:'Identity locked', value:state.meshHealth.security.identity_locked?'YES':'NO', accent:state.meshHealth.security.identity_locked?'var(--warn)':'var(--sig)' },
          ].map(m => (
            <div key={m.label} className="metric-card">
              <div className="ops-label" style={{ marginBottom:4 }}>{m.label}</div>
              <div style={{ fontFamily:'var(--mono)',fontSize:20,fontWeight:700,color:m.accent }}>{m.value}</div>
            </div>
          ))}
        </div>
      )}

      {/* Audit log */}
      <div className="ops-panel" style={{ padding:14 }}>
        <div className="ops-section-header">
          <span className="ops-section-title">Audit log</span>
          <div style={{ flex:1 }} />
          <button className="btn-ops" onClick={() => secApi.audit().then(setAuditLog).catch(()=>{})} style={{ padding:'3px 8px',fontSize:10 }}>
            ⟳ Refresh
          </button>
        </div>
        {auditLog.length === 0 ? (
          <div style={{ padding:'20px 0',textAlign:'center',fontFamily:'var(--mono)',fontSize:11,color:'var(--t4)' }}>
            No audit events recorded
          </div>
        ) : (
          <div style={{ maxHeight:300,overflowY:'auto' }}>
            {auditLog.map((entry, i) => (
              <div key={i} className="ops-row">
                <div style={{
                  width:5,height:5,borderRadius:'50%',flexShrink:0,
                  background:entry.level==='error'||entry.level==='critical'?'var(--crit)':entry.level==='warn'?'var(--warn)':'var(--info)'
                }} />
                <span style={{ fontFamily:'var(--mono)',fontSize:9,color:'var(--t3)',flexShrink:0,minWidth:60 }}>
                  {entry.time ? new Date(entry.time*1000).toLocaleTimeString() : '—'}
                </span>
                <span className={`ops-tag ops-tag-${entry.level==='error'||entry.level==='critical'?'crit':entry.level==='warn'?'warn':'info'}`} style={{ flexShrink:0,fontSize:9 }}>
                  {(entry.level||'info').toUpperCase()}
                </span>
                <span style={{ fontFamily:'var(--mono)',fontSize:11,color:'var(--t2)',flex:1,minWidth:0,overflow:'hidden',textOverflow:'ellipsis',whiteSpace:'nowrap' }}>
                  {entry.message || entry.event || JSON.stringify(entry)}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
