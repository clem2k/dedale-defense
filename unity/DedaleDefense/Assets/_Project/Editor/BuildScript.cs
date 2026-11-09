using System;
using System.IO;
using UnityEditor;
using UnityEditor.Build;
using UnityEditor.Build.Reporting;
using UnityEditor.SceneManagement;
using UnityEngine;

// Pas de namespace : Unity appelle « BuildScript.BuildLinux » via -executeMethod.
public static class BuildScript
{
    const string ScenePath = "Assets/_Project/Scenes/Game.unity";
    const string DefaultOutput = "Build/Linux/DedaleDefense.x86_64";

    /// <summary>Build natif Linux x86_64. Arguments : -output chemin, -il2cpp (sinon Mono).</summary>
    public static void BuildLinux()
    {
        try
        {
            ConfigurePlayer(HasArg("-il2cpp"));
            EnsureScene();

            string output = GetArg("-output") ?? DefaultOutput;
            Directory.CreateDirectory(Path.GetDirectoryName(output) ?? ".");

            var options = new BuildPlayerOptions
            {
                scenes = new[] { ScenePath },
                locationPathName = output,
                target = BuildTarget.StandaloneLinux64,
                options = BuildOptions.None,
            };
            BuildReport report = BuildPipeline.BuildPlayer(options);
            Debug.Log("Build : " + report.summary.result + " (" + report.summary.totalSize + " octets) -> " + output);
            EditorApplication.Exit(report.summary.result == BuildResult.Succeeded ? 0 : 1);
        }
        catch (Exception ex)
        {
            Debug.LogError(ex);
            EditorApplication.Exit(1);
        }
    }

    static void ConfigurePlayer(bool il2cpp)
    {
        PlayerSettings.companyName = "clem2k";
        PlayerSettings.productName = "Dédale Défense";
        PlayerSettings.SetApplicationIdentifier(NamedBuildTarget.Standalone, "com.clem2k.dedaledefense");
        PlayerSettings.defaultScreenWidth = 1280;
        PlayerSettings.defaultScreenHeight = 720;
        PlayerSettings.fullScreenMode = FullScreenMode.Windowed;
        PlayerSettings.resizableWindow = true;
        PlayerSettings.runInBackground = true;
        PlayerSettings.SetScriptingBackend(NamedBuildTarget.Standalone, il2cpp ? ScriptingImplementation.IL2CPP : ScriptingImplementation.Mono2x);
        Time.fixedDeltaTime = 1f / 60f;
    }

    /// <summary>Le jeu s'instancie tout seul (RuntimeInitializeOnLoadMethod) : une scène vide suffit.</summary>
    static void EnsureScene()
    {
        if (File.Exists(ScenePath)) return;
        Directory.CreateDirectory(Path.GetDirectoryName(ScenePath));
        var scene = EditorSceneManager.NewScene(NewSceneSetup.DefaultGameObjects, NewSceneMode.Single);
        EditorSceneManager.SaveScene(scene, ScenePath);
        AssetDatabase.Refresh();
    }

    static bool HasArg(string name) { return Array.IndexOf(Environment.GetCommandLineArgs(), name) >= 0; }

    static string GetArg(string name)
    {
        string[] a = Environment.GetCommandLineArgs();
        int i = Array.IndexOf(a, name);
        return i >= 0 && i + 1 < a.Length ? a[i + 1] : null;
    }
}
