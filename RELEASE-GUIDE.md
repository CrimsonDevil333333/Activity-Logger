# Creating Your First GitHub Release

## Why Was the Release Skipped?

The GitHub Actions workflow is configured to **only create releases when you push a version tag** (like `v1.0.0`). This is intentional to prevent creating releases on every commit.

**Line 115 in `.github/workflows/rust.yml`:**
```yaml
if: startsWith(github.ref, 'refs/tags/v')
```

This means:
- ✅ Regular commits: Build artifacts but **no release**
- ✅ Version tags (v1.0.0): Build artifacts **and create release**

## How to Create a Release

### Step 1: Commit Your Changes

Make sure all your changes are committed:

```bash
git add .
git commit -m "Release v1.0.0 - Initial release with web dashboard"
git push origin main
```

### Step 2: Create and Push a Version Tag

```bash
# Create a tag
git tag v1.0.0

# Push the tag to GitHub
git push origin v1.0.0
```

### Step 3: Watch the Magic Happen

1. Go to your repository on GitHub
2. Click on "Actions" tab
3. You'll see the workflow running
4. The "release" job will now execute (not skip!)
5. After completion, go to "Releases" tab
6. Your release will be there with downloadable packages!

## Version Tag Format

Use semantic versioning:
- `v1.0.0` - Major release
- `v1.1.0` - Minor update (new features)
- `v1.0.1` - Patch (bug fixes)

## What Gets Released

When you push a version tag, GitHub Actions will:

1. **Build** for Windows and Linux
2. **Package** with all necessary files:
   - Executable
   - config.json
   - README.md
   - Launcher script (run.bat/run.sh)
   - Icon
3. **Create ZIP** (Windows) and **tar.gz** (Linux)
4. **Create GitHub Release** with:
   - Release notes
   - Downloadable packages
   - Version tag

## Quick Command Reference

```bash
# Create release v1.0.0
git tag v1.0.0
git push origin v1.0.0

# Create release v1.1.0 (next version)
git tag v1.1.0
git push origin v1.1.0

# List all tags
git tag

# Delete a tag (if you made a mistake)
git tag -d v1.0.0                    # Delete locally
git push origin --delete v1.0.0      # Delete on GitHub
```

## Release Checklist

Before creating a release:

- [ ] All changes committed and pushed
- [ ] Version number updated in Cargo.toml (optional but recommended)
- [ ] README.md is up to date
- [ ] Tested the application locally
- [ ] Build script works (`.\build-package.ps1`)
- [ ] Ready to share with users!

## After Release

Once the release is created:

1. **Share the link**: `https://github.com/YOUR_USERNAME/Activity-Logger/releases`
2. **Users download**: They pick Windows or Linux package
3. **Users extract and run**: No installation needed!

## Example Release Notes

When GitHub creates the release, you can edit it to add release notes:

```markdown
# Activity Logger v1.0.0

## 🎉 Initial Release

A powerful activity logging tool with modern web dashboard!

### Features
- 📊 Real-time statistics dashboard
- 🔍 Search and filter logs
- 🗑️ Clear data with confirmation
- ⚙️ Web-based configuration editor
- 🔄 Auto-refresh (5s/10s/30s)
- 📸 Screenshot gallery
- 🎨 Modern dark theme UI

### Download
- **Windows**: Download `Activity-Logger-Windows.zip`
- **Linux**: Download `Activity-Logger-Linux.tar.gz`

### Quick Start
1. Extract the archive
2. Run `run.bat` (Windows) or `./run.sh` (Linux)
3. Open http://localhost:8080 in your browser

Enjoy! 🚀
```

## Troubleshooting

### Release job still skipped?
- Make sure tag starts with 'v' (e.g., `v1.0.0` not `1.0.0`)
- Check you pushed the tag: `git push origin v1.0.0`

### Build failed?
- Check the Actions tab for error details
- Ensure code builds locally: `cargo build --release`

### Can't push tag?
- Make sure you have write access to the repository
- Check if tag already exists: `git tag`

## Ready to Release?

Run these commands now:

```bash
# Make sure everything is committed
git status

# Create and push the tag
git tag v1.0.0
git push origin v1.0.0

# Then watch GitHub Actions create your release!
```

Visit: `https://github.com/YOUR_USERNAME/Activity-Logger/actions` to see it in action!
