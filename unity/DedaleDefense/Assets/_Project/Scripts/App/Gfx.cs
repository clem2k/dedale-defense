using UnityEngine;

namespace DedaleDefense.App
{
    /// <summary>Primitives de dessin IMGUI (rectangles, cercles, triangles) sans asset externe.</summary>
    public static class Gfx
    {
        static Texture2D circle, ring, tri;

        public static Color Hex(string hex, float alpha = 1f)
        {
            Color c;
            if (!ColorUtility.TryParseHtmlString(hex, out c)) c = Color.magenta;
            c.a = alpha;
            return c;
        }

        static Texture2D Make(int n, System.Func<float, float, float> alphaAt)
        {
            var t = new Texture2D(n, n, TextureFormat.RGBA32, false);
            t.filterMode = FilterMode.Bilinear;
            t.wrapMode = TextureWrapMode.Clamp;
            for (int y = 0; y < n; y++)
                for (int x = 0; x < n; x++)
                {
                    float u = (x + 0.5f) / n * 2f - 1f, v = (y + 0.5f) / n * 2f - 1f;
                    t.SetPixel(x, y, new Color(1, 1, 1, alphaAt(u, v)));
                }
            t.Apply();
            return t;
        }

        static void Init()
        {
            if (circle != null) return;
            const float aa = 0.04f;
            circle = Make(64, (u, v) => Mathf.Clamp01((1f - Mathf.Sqrt(u * u + v * v)) / aa));
            ring = Make(128, (u, v) =>
            {
                float d = Mathf.Sqrt(u * u + v * v);
                return Mathf.Clamp01((1f - d) / aa) * Mathf.Clamp01((d - 0.96f) / aa);
            });
            // Triangle pointant vers +x : sommet (1,0), base x=-1 entre y=-1 et y=1.
            tri = Make(64, (u, v) =>
            {
                float edge = (1f - u) / 2f;          // 0 au sommet, 1 à la base
                float half = edge;                   // demi-largeur autorisée
                return Mathf.Clamp01((half - Mathf.Abs(v)) / aa);
            });
        }

        public static void Rect(float x, float y, float w, float h, Color c)
        {
            Init();
            var old = GUI.color;
            GUI.color = c;
            GUI.DrawTexture(new Rect(x, y, w, h), Texture2D.whiteTexture);
            GUI.color = old;
        }

        public static void Circle(float cx, float cy, float r, Color c)
        {
            Init();
            var old = GUI.color;
            GUI.color = c;
            GUI.DrawTexture(new Rect(cx - r, cy - r, r * 2, r * 2), circle);
            GUI.color = old;
        }

        public static void Ring(float cx, float cy, float r, Color c)
        {
            Init();
            var old = GUI.color;
            GUI.color = c;
            GUI.DrawTexture(new Rect(cx - r, cy - r, r * 2, r * 2), ring);
            GUI.color = old;
        }

        /// <summary>Triangle de rayon r orienté selon angle (radians, 0 = vers +x).</summary>
        public static void Triangle(float cx, float cy, float r, float angle, Color c)
        {
            Init();
            var old = GUI.color;
            var m = GUI.matrix;
            GUIUtility.RotateAroundPivot(angle * Mathf.Rad2Deg, new Vector2(cx, cy));
            GUI.color = c;
            GUI.DrawTexture(new Rect(cx - r, cy - r, r * 2, r * 2), tri);
            GUI.matrix = m;
            GUI.color = old;
        }

        /// <summary>Rectangle centré en (cx,cy), tourné de angle radians.</summary>
        public static void RotRect(float cx, float cy, float w, float h, float angle, Color c)
        {
            Init();
            var old = GUI.color;
            var m = GUI.matrix;
            GUIUtility.RotateAroundPivot(angle * Mathf.Rad2Deg, new Vector2(cx, cy));
            GUI.color = c;
            GUI.DrawTexture(new Rect(cx - w / 2, cy - h / 2, w, h), Texture2D.whiteTexture);
            GUI.matrix = m;
            GUI.color = old;
        }

        public static void Line(float x0, float y0, float x1, float y1, float thick, Color c)
        {
            float dx = x1 - x0, dy = y1 - y0;
            float len = Mathf.Sqrt(dx * dx + dy * dy);
            if (len < 0.01f) return;
            RotRect((x0 + x1) / 2f, (y0 + y1) / 2f, len, thick, Mathf.Atan2(dy, dx), c);
        }

        public static void Label(float x, float y, float w, float h, string text, int size, Color c,
            TextAnchor anchor = TextAnchor.MiddleLeft, bool bold = false)
        {
            var style = new GUIStyle(GUI.skin.label);
            style.fontSize = size;
            style.alignment = anchor;
            style.fontStyle = bold ? FontStyle.Bold : FontStyle.Normal;
            style.normal.textColor = c;
            style.wordWrap = true;
            GUI.Label(new Rect(x, y, w, h), text, style);
        }
    }
}
