using System;
using System.Collections.Generic;
using Godot;

namespace NMines.scripts;

public partial class Game : Node3D
{
    [Export]
    public Mesh CellMesh;
    [Export]
    public Camera3D Camera;
    [Export]
    public string ScenePath;

    private int[,] _grid;
    private bool[,] _showedGrid;
    private bool[,] _flaggedGrid;
    private uint _flagged;
    private Rid _multimesh;
    private Rid _instance;
    private int _height;
    private int _width;
    private int _bombs;
    private int _originalBombs;
    private bool _generated;
    private ulong _start;
    
    public override void _Ready()
    {
        var gameSettings = GetNode("/root/GameSettings");
        _height = (int)gameSettings.Get("height");
        _width = (int)gameSettings.Get("width");
        _bombs = (int)gameSettings.Get("bombs");
        _multimesh = RenderingServer.MultimeshCreate();
        RenderingServer.MultimeshSetMesh(_multimesh, CellMesh.GetRid());
        RenderingServer.MultimeshAllocateData(_multimesh, _height * _width,
            RenderingServer.MultimeshTransformFormat.Transform3D, false, true);
        for (var z = 0; z < _height; z++)
        {
            for (var x = 0; x < _width; x++)
            {
                var index = z * _width + x;
                var transform = new Transform3D(Basis.Identity, new Vector3(x + 0.5f, 0.0f, z + 0.5f));
                RenderingServer.MultimeshInstanceSetTransform(_multimesh, index, transform);
                RenderingServer.MultimeshInstanceSetCustomData(_multimesh, index, new Color(0.0f, 0.0f, 1.0f, 12.0f));
            }
        }

        _instance = RenderingServer.InstanceCreate();
        RenderingServer.InstanceSetBase(_instance, _multimesh);
        RenderingServer.InstanceSetScenario(_instance, GetWorld3D().GetScenario());
        Camera.Position = new Vector3(_width / 2.0f, 15.0f, _height / 2.0f);
        var area = new Area3D();
        var collision = new CollisionShape3D();
        var shape = new BoxShape3D();
        shape.Size = new Vector3(_width, 0.0f, _height);
        collision.Shape = shape;
        collision.Position = new Vector3(_width / 2.0f, 0.0f, _height / 2.0f);
        area.AddChild(collision);
        AddChild(area);
        area.InputEvent += AreaOnInputEvent;
        _start = Time.GetTicksMsec();
    }

    private void AreaOnInputEvent(Node camera, InputEvent @event, Vector3 eventPosition, Vector3 normal, long shapeIdx)
    {
        switch (@event)
        {
            case InputEventMouseButton { ButtonIndex: MouseButton.Left, Pressed: true }:
            {
                var position = new Vector2I((int)Math.Floor(eventPosition.X), (int)Math.Floor(eventPosition.Z));
                if (!_generated)
                {
                    GenerateGrid(position);
                    _generated = true;
                }
                if (_grid[position.X, position.Y] == -1)
                {
                    PrepareLose();
                    return;
                }
                ShowCellAndNeighbours(position);
                break;
            }
            case InputEventMouseButton { ButtonIndex: MouseButton.Right, Pressed: true }:
            {
                var position = new Vector2I((int)Math.Floor(eventPosition.X), (int)Math.Floor(eventPosition.Z));
                if (!_showedGrid[position.X, position.Y])
                {
                    FlagCell(position);
                }

                break;
            }
        }
    }

    public override void _ExitTree()
    {
        RenderingServer.FreeRid(_instance);
        RenderingServer.FreeRid(_multimesh);
    }

    public override void _Process(double delta)
    {
        if (_generated) CheckWin();
    }

    private void PrepareLose()
    {
        var end = Time.GetTicksMsec();
        var timer = new Timer();
        timer.WaitTime = 2.0;
        timer.Autostart = true;
        timer.OneShot = true;
        timer.Timeout += () =>
        {
            EndGame(false, end);
        };

        for (var x = 0; x < _width; x++)
        {
            for (var y = 0; y < _height; y++)
            {
                if (_grid[x, y] != -1) continue;
                ShowCell(new Vector2I(x, y));
            }
        }
        AddChild(timer);
    }

    private void GenerateGrid(Vector2I clickPosition)
    {
        _grid = new int[_width, _height];
        _showedGrid = new bool[_width, _height];
        _flaggedGrid = new bool[_width, _height];
        for (var x = 0; x < _width; x++)
        {
            for (var y = 0; y < _height; y++)
            {
                _grid[x, y] = 0;
                _showedGrid[x, y] = false;
                _flaggedGrid[x, y] = false;
            }
        }

        var random = new Random();
        _originalBombs = _bombs;
        while (_bombs > 0)
        {
            var x = random.Next(0, _width);
            var y = random.Next(0, _height);
            if ((x == clickPosition.X && y == clickPosition.Y) || _grid[x,y] == -1) continue;
            _grid[x, y] = -1;
            for (var xOffset = -1; xOffset <= 1; xOffset++)
            {
                if (x == 0 && xOffset == -1) continue;
                if (x == _width - 1 && xOffset == 1) continue;
                for (var yOffset = -1; yOffset <= 1; yOffset++)
                {
                    if (y == 0 && yOffset == -1) continue;
                    if (y == _height - 1 && yOffset == 1) continue;
                    if (xOffset == 0 && yOffset == 0) continue;
                    if (_grid[x + xOffset, y + yOffset] == -1) continue;
                    _grid[x + xOffset, y + yOffset]++;
                }
            }

            _bombs--;
        }
    }

    private void ShowCellAndNeighbours(Vector2I cellPosition)
    {
        var positions = new List<Vector2I> { cellPosition };
        while (positions.Count > 0)
        {
            var position = positions[0];
            positions.RemoveAt(0);
            if (_showedGrid[position.X, position.Y]) continue;
            if (_flaggedGrid[position.X, position.Y]) continue;
            if (_grid[position.X, position.Y] == -1) continue;
            ShowCell(position);
            if (_grid[position.X, position.Y] != 0) continue;
            for (var xOffset = -1; xOffset <= 1; xOffset++)
            {
                if (position.X == 0 && xOffset == -1) continue;
                if (position.X == _width - 1 && xOffset == 1) continue;
                for (var yOffset = -1; yOffset <= 1; yOffset++)
                {
                    if (position.Y == 0 && yOffset == -1) continue;
                    if (position.Y == _height - 1 && yOffset == 1) continue;
                    if (xOffset == 0 && yOffset == 0) continue;
                    if (_grid[position.X + xOffset, position.Y + yOffset] == -1) continue;
                    if (_showedGrid[position.X + xOffset, position.Y + yOffset]) continue;
                    positions.Add(new Vector2I(position.X + xOffset, position.Y + yOffset));
                }
            }
        }
    }
    
    private void ShowCell(Vector2I cellPosition)
    {
        var value = _grid[cellPosition.X, cellPosition.Y];
        var index = cellPosition.Y * _width + cellPosition.X;
        var cell = value switch
        {
            0 => 1.0f,
            -1 => 2.0f,
            _ => 3.0f + value
        };
        RenderingServer.MultimeshInstanceSetCustomData(_multimesh, index, new Color(0.0f, cell, 1.0f, 12.0f));
        _showedGrid[cellPosition.X, cellPosition.Y] = true;
    }

    private void FlagCell(Vector2I cellPosition)
    {
        if (_showedGrid[cellPosition.X, cellPosition.Y]) return;
        var index = cellPosition.Y * _width + cellPosition.X;
        _flaggedGrid[cellPosition.X, cellPosition.Y] = !_flaggedGrid[cellPosition.X, cellPosition.Y];
        float cell;
        if (_flaggedGrid[cellPosition.X, cellPosition.Y])
        {
            cell = 3.0f;
            _flagged++;
        }
        else
        {
            cell = 0.0f;
            _flagged--;
        }
        RenderingServer.MultimeshInstanceSetCustomData(_multimesh, index, new Color(0.0f, cell, 1.0f, 12.0f));
    }

    private void CheckWin()
    {
        if (_flagged != _originalBombs) return;
        for (var x = 0; x < _width; x++)
        {
            for (var y = 0; y < _height; y++)
            {
                if (_grid[x, y] != -1 && !_showedGrid[x, y]) return;
                if (_grid[x, y] == -1 && !_flaggedGrid[x,y]) return;
            }
        }
        EndGame(true);
    }

    private void EndGame(bool win, ulong end = 0)
    {
        if (end == 0) end = Time.GetTicksMsec();
        var gameSettings = GetNode("/root/GameSettings");
        gameSettings.Set("win", win);
        gameSettings.Set("timer", end - _start);
        GetTree().ChangeSceneToFile(ScenePath);
    }
}