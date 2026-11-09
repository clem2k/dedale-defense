using DedaleDefense.Core;
using UnityEngine;

namespace DedaleDefense.App
{
    /// <summary>Record par mode, équivalent du localStorage « dd-best-&lt;mode&gt; » de la version web.</summary>
    public sealed class PrefsBestStore : IBestStore
    {
        public int Get(string key) { return PlayerPrefs.GetInt(key, 0); }
        public void Set(string key, int value) { PlayerPrefs.SetInt(key, value); PlayerPrefs.Save(); }
    }
}
